import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { NavBar, Swiper, Tag, List, DotLoading } from "antd-mobile";
import { ChevronLeft } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { pieceRecordApi, orderApi, processApi } from "@/api";
import type { PieceRecordStatus, RecordedBy } from "@/types";
import dayjs from "dayjs";

export const Route = createFileRoute("/_auth/_staff/my-records/$id")({
  component: StaffRecordDetailPage,
});

const STATUS_MAP: Record<PieceRecordStatus, { label: string; color: string }> =
  {
    pending: { label: "待审核", color: "#faad14" },
    approved: { label: "已通过", color: "#52c41a" },
    rejected: { label: "已拒绝", color: "#ff4d4f" },
    settled: { label: "已结算", color: "#722ed1" },
  };

const RECORDED_BY_MAP: Record<RecordedBy, string> = {
  bySelf: "自己录入",
  byBoss: "老板代录",
};

function StaffRecordDetailPage() {
  const navigate = useNavigate();
  const { id } = Route.useParams();

  const { data: record, isLoading: recordLoading } = useQuery({
    queryKey: ["piece-record", id],
    queryFn: () => pieceRecordApi.getOne(id),
  });

  const { data: process, isLoading: processLoading } = useQuery({
    queryKey: ["process", record?.processId],
    queryFn: () => processApi.getOne(record!.processId),
    enabled: !!record?.processId,
  });

  const { data: order, isLoading: orderLoading } = useQuery({
    queryKey: ["order", process?.orderId],
    queryFn: () => orderApi.getOne(process!.orderId),
    enabled: !!process?.orderId,
  });

  const isLoading = recordLoading || processLoading || orderLoading;

  if (isLoading) {
    return (
      <div className="flex flex-col h-full">
        <NavBar onBack={() => navigate({ to: "/my-records" })} backIcon={<ChevronLeft size={24} />}>
          计件详情
        </NavBar>
        <div className="flex-1 flex items-center justify-center">
          <DotLoading color="primary" />
        </div>
      </div>
    );
  }

  if (!record) {
    return (
      <div className="flex flex-col h-full">
        <NavBar onBack={() => navigate({ to: "/my-records" })} backIcon={<ChevronLeft size={24} />}>
          计件详情
        </NavBar>
        <div className="flex-1 flex items-center justify-center text-gray-500">
          记录不存在
        </div>
      </div>
    );
  }

  const images = order?.images ?? [];

  console.log(record, process, order);

  return (
    <div className="flex flex-col h-full bg-gray-50">
      <NavBar onBack={() => navigate({ to: "/my-records" })} backIcon={<ChevronLeft size={24} />}>
        计件详情
      </NavBar>

      <div className="flex-1 overflow-auto">
        {images.length > 0 && (
          <Swiper autoplay={images.length > 1} loop={images.length > 1}>
            {images.map((img, index) => (
              <Swiper.Item key={index}>
                <div className="h-56 bg-gray-200">
                  <img
                    src={img}
                    alt={`订单图片 ${index + 1}`}
                    className="w-full h-full object-contain"
                  />
                </div>
              </Swiper.Item>
            ))}
          </Swiper>
        )}

        <div className="m-3">
          <div className="bg-white rounded-lg mb-3">
            <div className="p-3 border-b border-gray-100 font-medium">
              订单信息
            </div>
            <List>
              <List.Item extra={order?.productName || "-"}>产品名称</List.Item>
              {order?.description && (
                <List.Item extra={order.description}>描述</List.Item>
              )}
              <List.Item extra={order?.quantity || "-"}>订单数量</List.Item>
              <List.Item
                extra={
                  order?.status && (
                    <Tag color="primary" fill="outline">
                      {order.status === "pending"
                        ? "待开工"
                        : order.status === "processing"
                        ? "进行中"
                        : order.status === "completed"
                        ? "已完工"
                        : order.status === "delivered"
                        ? "已交货"
                        : "已取消"}
                    </Tag>
                  )
                }
              >
                订单状态
              </List.Item>
            </List>
          </div>

          <div className="bg-white rounded-lg mb-3">
            <div className="p-3 border-b border-gray-100 font-medium">
              工序信息
            </div>
            <List>
              <List.Item extra={process?.name || "-"}>工序名称</List.Item>
              {process?.description && (
                <List.Item extra={process.description}>描述</List.Item>
              )}
              <List.Item
                extra={
                  <span className="text-orange-500">
                    ¥{process?.piecePrice || "0"}
                  </span>
                }
              >
                计件单价
              </List.Item>
            </List>
          </div>

          <div className="bg-white rounded-lg">
            <div className="p-3 border-b border-gray-100 font-medium">
              计件信息
            </div>
            <List>
              <List.Item extra={record.quantity}>计件数量</List.Item>
              <List.Item
                extra={
                  <span className="text-orange-500 font-medium">
                    ¥{record.amount}
                  </span>
                }
              >
                计件金额
              </List.Item>
              <List.Item
                extra={
                  <Tag
                    color={STATUS_MAP[record.status].color}
                    fill="outline"
                    style={{ "--border-radius": "4px" }}
                  >
                    {STATUS_MAP[record.status].label}
                  </Tag>
                }
              >
                审核状态
              </List.Item>
              <List.Item extra={RECORDED_BY_MAP[record.recordedBy]}>
                录入方式
              </List.Item>
              <List.Item
                extra={dayjs(record.recordedAt).format("YYYY-MM-DD HH:mm")}
              >
                录入时间
              </List.Item>
            </List>
          </div>
        </div>
      </div>
    </div>
  );
}
