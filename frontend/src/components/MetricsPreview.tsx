import { Label, ProgressBar } from "@heroui/react";

export interface Metrics {
  cpu: number;
  ram: { used: number; total: number };
  disk: { used: number; total: number };
  network: { in: number; out: number };
}

export interface MetricsPreviewProps {
  metrics: Metrics;
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  const units = ["KB", "MB", "GB", "TB"];
  let v = bytes / 1024;
  let i = 0;
  while (v >= 1024 && i < units.length - 1) {
    v /= 1024;
    i++;
  }
  return `${v.toFixed(v < 10 ? 1 : 0)} ${units[i]}`;
}

function formatRate(bytesPerSec: number): string {
  return `${formatBytes(bytesPerSec)}/s`;
}

function pct(used: number, total: number): number {
  return total > 0 ? Math.round((used / total) * 100) : 0;
}

function colorFor(percent: number): "success" | "warning" | "danger" {
  if (percent >= 90) return "danger";
  if (percent >= 70) return "warning";
  return "success";
}

function Row({
  label,
  value,
  percent,
}: {
  label: string;
  value: string;
  percent: number;
}) {
  return (
    <ProgressBar
      aria-label={label}
      className="w-full"
      value={percent}
      color={colorFor(percent)}
      size="sm"
    >
      <div className="flex justify-between text-xs">
        <Label className="text-foreground/70">{label}</Label>
        <span className="tabular-nums text-foreground/80">{value}</span>
      </div>
      <ProgressBar.Track>
        <ProgressBar.Fill />
      </ProgressBar.Track>
    </ProgressBar>
  );
}

export function MetricsPreview({ metrics }: MetricsPreviewProps) {
  const cpuPct = Math.round(metrics.cpu);
  const ramPct = pct(metrics.ram.used, metrics.ram.total);
  const diskPct = pct(metrics.disk.used, metrics.disk.total);

  return (
    <div className="flex flex-col gap-2.5">
      <Row label="CPU" value={`${cpuPct}%`} percent={cpuPct} />
      <Row
        label="RAM"
        value={`${formatBytes(metrics.ram.used)} / ${formatBytes(metrics.ram.total)}`}
        percent={ramPct}
      />
      <Row
        label="Disk"
        value={`${formatBytes(metrics.disk.used)} / ${formatBytes(metrics.disk.total)}`}
        percent={diskPct}
      />
      <div className="flex justify-between text-xs pt-0.5">
        <span className="text-foreground/70">Network</span>
        <span className="tabular-nums text-foreground/80 flex gap-3">
          <span>↓ {formatRate(metrics.network.in)}</span>
          <span>↑ {formatRate(metrics.network.out)}</span>
        </span>
      </div>
    </div>
  );
}

export default MetricsPreview;
