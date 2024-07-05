import { z } from 'zod';

export const ShaMiningSettingsSchema = z.object({
  tariAddress: z.string(),
  shaThreads: z.number(),
  shaMineOnStartup: z.boolean(),
});

export const MergedMiningSettingsSchema = z.object({
  moneroAddress: z.string(),
  moneroNodeUrl: z.string(),
  mergeMineOnStartup: z.boolean(),
});

export const BaseNodeSettingsSchema = z.object({
  network: z.string(),
  rootFolder: z.string(),
  runOnStartup: z.boolean(),
});


export const FormDataSchema = z.object({
  mergedMiningSettings: MergedMiningSettingsSchema,
  baseNodeSettings: BaseNodeSettingsSchema,
  shaMiningSettings: ShaMiningSettingsSchema,
});

export type ShaMiningSettingsType = z.infer<typeof ShaMiningSettingsSchema>;
export type MergedMiningSettingsType = z.infer<
  typeof MergedMiningSettingsSchema
>;
export type BaseNodeSettingsType = z.infer<typeof BaseNodeSettingsSchema>;
export type FormDataType = z.infer<typeof FormDataSchema>;

export interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}
