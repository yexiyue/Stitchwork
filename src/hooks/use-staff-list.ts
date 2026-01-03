import { useQuery } from "@tanstack/react-query";
import { authApi } from "@/api";

export const useStaffList = () =>
  useQuery({
    queryKey: ["staff-list"],
    queryFn: () => authApi.getStaffList(),
  });
