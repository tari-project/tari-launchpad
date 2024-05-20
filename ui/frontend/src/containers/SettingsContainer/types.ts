import { z } from 'zod';

export const MiningSettingsSchema = z.object({
  shaThreads: z.number(),
  moneroAddress: z.string(),
  randomXThreads: z.number(),
  moneroNodeUrl: z.string(),
  // walletPaymentAddress: z.any(),
});

export const BaseNodeSettingsSchema = z.object({
  network: z.string(),
  rootFolder: z.string(),
});

export const WalletSettingsSchema = z.object({
  tariAddress: z.string(),
});

export const DockerSettingsSchema = z.object({
  dockerTag: z.string(),
  dockerRegistry: z.string(),
});

// export const GeneralSettingsSchema = z.object({
//   runOnStartup: z.boolean(),
//   mineOnStartup: z.boolean(),
// });

export const FormDataSchema = z.object({
  miningSettings: MiningSettingsSchema,
  baseNodeSettings: BaseNodeSettingsSchema,
  walletSettings: WalletSettingsSchema,
  dockerSettings: DockerSettingsSchema,
  // generalSettings: GeneralSettingsSchema,
});

export type MiningSettingsType = z.infer<typeof MiningSettingsSchema>;
export type BaseNodeSettingsType = z.infer<typeof BaseNodeSettingsSchema>;
export type WalletSettingsType = z.infer<typeof WalletSettingsSchema>;
export type DockerSettingsType = z.infer<typeof DockerSettingsSchema>;
// export type GeneralSettingsType = z.infer<typeof GeneralSettingsSchema>;
export type FormDataType = z.infer<typeof FormDataSchema>;

export interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}
