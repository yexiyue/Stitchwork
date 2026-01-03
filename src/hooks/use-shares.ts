import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { shareApi } from "@/api";
import type { CreateShareRequest, UpdateShareRequest } from "@/types";

const KEY = "shares";

export const useShares = () =>
  useQuery({
    queryKey: [KEY],
    queryFn: () => shareApi.list(),
  });

export const useCreateShare = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateShareRequest) => shareApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: [KEY] }),
  });
};

export const useUpdateShare = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateShareRequest }) => shareApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: [KEY] }),
  });
};

export const useDeleteShare = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => shareApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: [KEY] }),
  });
};

// 公开分享（无需认证）
export const usePublicShare = (token: string) =>
  useQuery({
    queryKey: ["public-share", token],
    queryFn: () => shareApi.getPublic(token),
    enabled: !!token,
  });
