// ECharts with tree-shaking support
import * as echarts from "echarts/core";
import { BarChart, PieChart, LineChart } from "echarts/charts";
import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent,
  DataZoomComponent,
} from "echarts/components";
import { CanvasRenderer } from "echarts/renderers";

// Register required components
echarts.use([
  BarChart,
  PieChart,
  LineChart,
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent,
  DataZoomComponent,
  CanvasRenderer,
]);

export default echarts;
