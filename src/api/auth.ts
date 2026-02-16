import { client, setToken } from "./client";
import type {
  LoginRequest,
  LoginResponse,
  LoginUser,
  RegisterRequest,
  UpdateProfileRequest,
  ChangePasswordRequest,
  Staff,
  ListData,
  QueryParams,
  Workshop,
  CreateWorkshopRequest,
  UpdateWorkshopRequest,
} from "@/types";

export const authApi = {
  login: async (data: LoginRequest) => {
    const res = await client.post<LoginResponse>("/api/login", data);
    setToken(res.token);
    return res;
  },
  register: async (data: RegisterRequest) => {
    const res = await client.post<LoginResponse>("/api/register", data);
    setToken(res.token);
    return res;
  },
  updateProfile: (data: UpdateProfileRequest) =>
    client.put<void>("/api/profile", data),
  changePassword: (data: ChangePasswordRequest) =>
    client.put<void>("/api/password", data),
  getStaffList: (params?: QueryParams) =>
    client.get<ListData<Staff>>("/api/staff", { params }),
  removeStaff: (staffId: string) => client.delete<void>(`/api/staff/${staffId}`),
  getProfile: () => client.get<LoginUser>("/api/profile"),
  // Workshop
  getWorkshop: () => client.get<Workshop | null>("/api/workshop"),
  createWorkshop: (data: CreateWorkshopRequest) =>
    client.post<Workshop>("/api/workshop", data),
  updateWorkshop: (data: UpdateWorkshopRequest) =>
    client.put<Workshop>("/api/workshop", data),
};
