import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import type { ListData, QueryParams } from "@/types";

interface CrudApi<T, C, U> {
  list: (params?: QueryParams) => Promise<ListData<T>>;
  getOne: (id: string) => Promise<T>;
  create: (data: C) => Promise<T>;
  update: (id: string, data: U) => Promise<T>;
  delete: (id: string) => Promise<void>;
}

export function createCrudHooks<T, C, U>(key: string, api: CrudApi<T, C, U>) {
  return {
    useList: (params?: QueryParams) =>
      useQuery({
        queryKey: params ? [key, params] : [key],
        queryFn: () => api.list(params),
      }),

    useOne: (id: string) =>
      useQuery({
        queryKey: [key, id],
        queryFn: () => api.getOne(id),
        enabled: !!id,
      }),

    useCreate: () => {
      const qc = useQueryClient();
      return useMutation({
        mutationFn: (data: C) => api.create(data),
        onSuccess: () => qc.invalidateQueries({ queryKey: [key] }),
      });
    },

    useUpdate: () => {
      const qc = useQueryClient();
      return useMutation({
        mutationFn: ({ id, data }: { id: string; data: U }) => api.update(id, data),
        onSuccess: () => qc.invalidateQueries({ queryKey: [key] }),
      });
    },

    useDelete: () => {
      const qc = useQueryClient();
      return useMutation({
        mutationFn: (id: string) => api.delete(id),
        onSuccess: () => qc.invalidateQueries({ queryKey: [key] }),
      });
    },
  };
}
