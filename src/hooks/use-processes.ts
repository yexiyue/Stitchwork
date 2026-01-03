import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { processApi } from "@/api";
import type {
  CreateProcessDto,
  UpdateProcessDto,
  QueryParams,
  ProcessQueryParams,
} from "@/types";

const KEY = "processes";

export const useProcesses = (params?: QueryParams & ProcessQueryParams) =>
  useQuery({
    queryKey: params ? [KEY, params] : [KEY],
    queryFn: () => processApi.list(params),
  });

export const useProcess = (id: string) =>
  useQuery({
    queryKey: [KEY, id],
    queryFn: () => processApi.getOne(id),
    enabled: !!id,
  });

export const useCreateProcess = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateProcessDto) => processApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: [KEY] }),
  });
};

export const useUpdateProcess = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateProcessDto }) =>
      processApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: [KEY] }),
  });
};

export const useDeleteProcess = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => processApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: [KEY] }),
  });
};
