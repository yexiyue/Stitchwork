export interface LoginRequest {
  username: string;
  password: string;
}

export type Role = "boss" | "staff";

export interface Workshop {
  id: string;
  name: string;
  desc?: string;
  address?: string;
  image?: string;
}

export interface LoginUser {
  id: string;
  username: string;
  role: Role;
  displayName?: string;
  phone: string;
  avatar?: string;
  isSuperAdmin: boolean;
  workshop?: Workshop;
}

export interface LoginResponse {
  token: string;
  user: LoginUser;
}

export interface RegisterRequest {
  username: string;
  password: string;
  phone: string;
  registerCode: string;
}

export interface RegisterStaffRequest {
  username: string;
  password: string;
  phone: string;
  inviteCode: string;
}

export interface InviteCodeResponse {
  code: string;
  expiresAt: number;
}

export interface BindWorkshopRequest {
  inviteCode: string;
}

export interface UpdateProfileRequest {
  displayName?: string;
  phone?: string;
  avatar?: string;
}

export interface ChangePasswordRequest {
  oldPassword: string;
  newPassword: string;
}

export interface CreateWorkshopRequest {
  name: string;
  desc?: string;
  address?: string;
  image?: string;
}

export interface UpdateWorkshopRequest {
  name?: string;
  desc?: string;
  address?: string;
  image?: string;
}

export interface Staff {
  id: string;
  username: string;
  displayName?: string;
  phone?: string;
  avatar?: string;
  createdAt?: string;
}
