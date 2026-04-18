import { useState, type ReactNode } from "react";
import {
  Button,
  Form,
  Input,
  Label,
  Modal,
  Radio,
  RadioGroup,
  TextArea,
  TextField,
} from "@heroui/react";

export type AuthMethod = "key" | "password";

export interface ConnectServerValues {
  ip: string;
  port: string;
  auth: AuthMethod;
  privateKey?: string;
  username?: string;
  password?: string;
}

export interface ConnectServerModalProps {
  children: ReactNode;
  onSubmit?: (values: ConnectServerValues) => void;
}

export function ConnectServerModal({
  children,
  onSubmit,
}: ConnectServerModalProps) {
  const [auth, setAuth] = useState<AuthMethod>("key");

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const data = new FormData(e.currentTarget);
    onSubmit?.({
      ip: String(data.get("ip") ?? ""),
      port: String(data.get("port") ?? ""),
      auth,
      privateKey:
        auth === "key" ? String(data.get("privateKey") ?? "") : undefined,
      username:
        auth === "password" ? String(data.get("username") ?? "") : undefined,
      password:
        auth === "password" ? String(data.get("password") ?? "") : undefined,
    });
  };

  return (
    <Modal>
      {children}
      <Modal.Backdrop variant="blur">
        <Modal.Container>
          <Modal.Dialog className="sm:max-w-[420px]">
            <Modal.CloseTrigger />
            <Modal.Header>
              <Modal.Heading>Connect Server</Modal.Heading>
            </Modal.Header>
            <Modal.Body>
              <Form
                id="connect-server-form"
                onSubmit={handleSubmit}
                className="flex flex-col gap-4"
              >
                <div className="flex gap-3">
                  <TextField
                    variant="secondary"
                    isRequired
                    name="ip"
                    className="flex-1"
                  >
                    <Label>IP Address</Label>
                    <Input placeholder="192.168.1.10" />
                  </TextField>
                  <TextField
                    variant="secondary"
                    isRequired
                    name="port"
                    type="number"
                    defaultValue="22"
                    className="w-24"
                  >
                    <Label>Port</Label>
                    <Input placeholder="22" />
                  </TextField>
                </div>

                <RadioGroup
                  variant="secondary"
                  name="auth"
                  value={auth}
                  onChange={(v) => setAuth(v as AuthMethod)}
                  orientation="horizontal"
                >
                  <Label>Authentication</Label>
                  <Radio value="key">
                    <Radio.Control>
                      <Radio.Indicator />
                    </Radio.Control>
                    <Radio.Content>
                      <Label>SSH Key</Label>
                    </Radio.Content>
                  </Radio>
                  <Radio value="password">
                    <Radio.Control>
                      <Radio.Indicator />
                    </Radio.Control>
                    <Radio.Content>
                      <Label>Password</Label>
                    </Radio.Content>
                  </Radio>
                </RadioGroup>

                {auth === "key" ? (
                  <TextField isRequired name="privateKey">
                    <Label>Private Key</Label>
                    <TextArea
                      variant="secondary"
                      placeholder="-----BEGIN OPENSSH PRIVATE KEY-----"
                      rows={5}
                      className="font-mono text-xs"
                    />
                  </TextField>
                ) : (
                  <div className="flex flex-col gap-3">
                    <TextField isRequired name="username">
                      <Label>Username</Label>
                      <Input variant="secondary" placeholder="root" />
                    </TextField>
                    <TextField isRequired name="password" type="password">
                      <Label>Password</Label>
                      <Input variant="secondary" placeholder="••••••••" />
                    </TextField>
                  </div>
                )}
              </Form>
            </Modal.Body>
            <Modal.Footer>
              <Button
                className="w-full"
                type="submit"
                form="connect-server-form"
                slot="close"
              >
                Connect
              </Button>
            </Modal.Footer>
          </Modal.Dialog>
        </Modal.Container>
      </Modal.Backdrop>
    </Modal>
  );
}

export default ConnectServerModal;
