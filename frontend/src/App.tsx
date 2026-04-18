import { Button, useTheme } from "@heroui/react";
import Logo from "./assets/logo.svg?react";

import { Card, Avatar } from "@heroui/react";
import { Status } from "./components/Status";
import { MetricsPreview } from "./components/MetricsPreview";
import { ConnectServerModal } from "./components/ConnectServerModal";


const GB = 1024 * 1024 * 1024;
const MB = 1024 * 1024;

const sampleMetrics = {
  cpu: 38,
  ram: { used: 2.3 * GB, total: 8 * GB },
  disk: { used: 42 * GB, total: 160 * GB },
  network: { in: 340 * 1024, out: 1.2 * MB },
};

function App() {
  useTheme("dark");

  return (
    <main className="mx-auto container py-4 space-y-8 px-4 sm:px-0">
      <div className="flex justify-between items-center">
        <Logo className="text-white h-8 w-min" />
        <ConnectServerModal onSubmit={(v) => console.log("connect", v)}>
          <Button>Connect Server</Button>
        </ConnectServerModal>
      </div>
      <div className="grid grid-cols-1 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
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
                <Card.Title>Proxyon MISC</Card.Title>
                <Status variant="online" />
              </div>
              <Card.Description>Debian</Card.Description>
            </div>
          </Card.Header>
          <Card.Content>
            <MetricsPreview metrics={sampleMetrics} />
          </Card.Content>
        </Card>
      </div>
    </main>
  );
}

export default App;
