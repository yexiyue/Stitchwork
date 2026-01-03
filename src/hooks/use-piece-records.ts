import { createCrudHooks } from "./create-crud-hooks";
import { pieceRecordApi } from "@/api";
import type { PieceRecord, CreatePieceRecordDto, UpdatePieceRecordDto } from "@/types";

const hooks = createCrudHooks<PieceRecord, CreatePieceRecordDto, UpdatePieceRecordDto>("piece-records", pieceRecordApi);

export const usePieceRecords = hooks.useList;
export const usePieceRecord = hooks.useOne;
export const useCreatePieceRecord = hooks.useCreate;
export const useUpdatePieceRecord = hooks.useUpdate;
export const useDeletePieceRecord = hooks.useDelete;
