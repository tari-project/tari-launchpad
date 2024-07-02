import { z } from 'zod';

export const ShaMiningSettingsSchema = z.object({
  tariAddress: z.string(),
  shaThreads: z.number(),
});

export const MergedMiningSettingsSchema = z.object({
  moneroAddress: z.string(),
  randomXThreads: z.number(),
  moneroNodeUrl: z.string(),
});

export const BaseNodeSettingsSchema = z.object({
  network: z.string(),
  rootFolder: z.string(),
  runOnStartup: z.boolean(),
});

export const DockerSettingsSchema = z.object({
  dockerTag: z.string(),
  dockerRegistry: z.string(),
});

export const FormDataSchema = z.object({
  mergedMiningSettings: MergedMiningSettingsSchema,
  baseNodeSettings: BaseNodeSettingsSchema,
  shaMiningSettings: ShaMiningSettingsSchema,
  dockerSettings: DockerSettingsSchema,
});

export type ShaMiningSettingsType = z.infer<typeof ShaMiningSettingsSchema>;
export type MergedMiningSettingsType = z.infer<
  typeof MergedMiningSettingsSchema
>;
export type BaseNodeSettingsType = z.infer<typeof BaseNodeSettingsSchema>;
export type DockerSettingsType = z.infer<typeof DockerSettingsSchema>;
export type FormDataType = z.infer<typeof FormDataSchema>;

export interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}
