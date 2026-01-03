import { client, setToken } from "./client";
import type {
  LoginRequest,
  LoginResponse,
  LoginUser,
  RegisterRequest,
  InviteCodeResponse,
  BindWorkshopRequest,
  UpdateProfileRequest,
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
  register: (data: RegisterRequest) =>
    client.post<{ userId: string }>("/api/register", data),
  generateInviteCode: () =>
    client.post<InviteCodeResponse>("/api/invite-code"),
  bindWorkshop: (data: BindWorkshopRequest) =>
    client.post<void>("/api/bind-workshop", data),
  updateProfile: (data: UpdateProfileRequest) =>
    client.put<void>("/api/profile", data),
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
