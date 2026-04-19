import { Button, useTheme } from "@heroui/react";
import Logo from "./assets/logo.svg?react";

import { Card, Avatar } from "@heroui/react";
import { Status } from "./components/Status";
import { MetricsPreview } from "./components/MetricsPreview";
import { ConnectServerModal } from "./components/ConnectServerModal";
import { useEffect, useState } from "react";
import { type ServerPublic } from "./bindings/ServerPublic";

function App() {
  useTheme("dark");

  const [servers, setServers] = useState<ServerPublic[]>([]);

  async function fetchServers() {
    const res = await fetch("/api/servers");
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = await res.json();
    setServers(data);
  }

  useEffect(() => {
    fetchServers();
  }, []);

  return (
    <main className="mx-auto container py-4 space-y-8 px-4 sm:px-0">
      <div className="flex justify-between items-center">
        <Logo className="text-white h-8 w-min" />
        <ConnectServerModal onSubmit={(v) => console.log("connect", v)}>
          <Button>Connect Server</Button>
        </ConnectServerModal>
      </div>
      <div className="grid grid-cols-1 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {servers.map((server) => {
          return (
            <Card>
              <Card.Header className="flex flex-row gap-2">
                <Avatar className="rounded-lg">
                  <Avatar.Image
                    alt="Debian"
                    src="https://heroui-assets.nyc3.cdn.digitaloceanspaces.com/avatars/blue.jpg"
                  />
                  <Avatar.Fallback>PM</Avatar.Fallback>
                </Avatar>
                <div>
                  <div className="flex items-center gap-2">
                    <Card.Title>{server.name}</Card.Title>
                    <Status variant="online" />
                  </div>
                  <Card.Description>Debian</Card.Description>
                </div>
              </Card.Header>
              <Card.Content>
                <MetricsPreview
                  metrics={{
                    cpu: server.cpu ?? 0,
                    ram: {
                      used: Number(server.ram_used ?? 0n),
                      total: Number(server.ram_total ?? 0n),
                    },
                    disk: {
                      used: Number(server.disk_used ?? 0n),
                      total: Number(server.disk_total ?? 0n),
                    },
                    network: {
                      in: Number(server.net_rx ?? 0n),
                      out: Number(server.net_tx ?? 0n),
                    },
                  }}
                />
              </Card.Content>
            </Card>
          );
        })}
      </div>
    </main>
  );
}

export default App;
