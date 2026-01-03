import { useMutation } from "@tanstack/react-query";
import { authApi } from "@/api";
import type { LoginRequest, RegisterRequest, CreateStaffRequest, BindBossRequest, UpdateProfileRequest } from "@/types";

export const useLogin = () =>
  useMutation({ mutationFn: (data: LoginRequest) => authApi.login(data) });

export const useRegister = () =>
  useMutation({ mutationFn: (data: RegisterRequest) => authApi.register(data) });

export const useCreateStaff = () =>
  useMutation({ mutationFn: (data: CreateStaffRequest) => authApi.createStaff(data) });

export const useGenerateInviteCode = () =>
  useMutation({ mutationFn: () => authApi.generateInviteCode() });

export const useBindBoss = () =>
  useMutation({ mutationFn: (data: BindBossRequest) => authApi.bindBoss(data) });

export const useUpdateProfile = () =>
  useMutation({ mutationFn: (data: UpdateProfileRequest) => authApi.updateProfile(data) });
