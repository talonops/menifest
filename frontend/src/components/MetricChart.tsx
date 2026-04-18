import {
  Area,
  AreaChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";

interface TooltipPayload {
  active?: boolean;
  payload?: Array<{ value: number; payload: MetricPoint }>;
  label?: string;
}

function ChartTooltip({
  active,
  payload,
  label,
  color,
  unit,
}: TooltipPayload & { color: string; unit: string }) {
  if (!active || !payload?.length) return null;
  const value = payload[0].value;

  return (
    <div className="rounded-md border border-white/10 bg-black/80 backdrop-blur px-2.5 py-1.5 text-xs shadow-lg">
      <div className="text-white/50 text-[10px] uppercase tracking-wide">
        {label}
      </div>
      <div className="font-semibold tabular-nums" style={{ color }}>
        {value}
        {unit}
      </div>
    </div>
  );
}

export interface MetricPoint {
  t: string;
  value: number;
}

export interface MetricChartProps {
  data: MetricPoint[];
  color?: string;
  height?: number;
  unit?: string;
}

export function MetricChart({
  data,
  color = "oklch(0.72 0.17 150)",
  height = 120,
  unit = "%",
}: MetricChartProps) {
  const gradientId = `metric-gradient-${color.replace(/[^a-z0-9]/gi, "")}`;

  return (
    <ResponsiveContainer width="100%" height={height}>
      <AreaChart data={data} margin={{ top: 4, right: 4, left: 0, bottom: 0 }}>
        <defs>
          <linearGradient id={gradientId} x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stopColor={color} stopOpacity={0.5} />
            <stop offset="100%" stopColor={color} stopOpacity={0} />
          </linearGradient>
        </defs>
        <XAxis
          dataKey="t"
          tick={{ fill: "currentColor", fontSize: 10, opacity: 0.6 }}
          axisLine={false}
          tickLine={false}
        />
        <YAxis
          tick={{ fill: "currentColor", fontSize: 10, opacity: 0.6 }}
          axisLine={false}
          tickLine={false}
          width={28}
          unit={unit}
        />
        <Tooltip
          cursor={{ stroke: color, strokeOpacity: 0.3, strokeWidth: 1 }}
          content={<ChartTooltip color={color} unit={unit} />}
        />
        <Area
          type="monotone"
          dataKey="value"
          stroke={color}
          strokeWidth={2}
          fill={`url(#${gradientId})`}
          isAnimationActive
        />
      </AreaChart>
    </ResponsiveContainer>
  );
}

export default MetricChart;
