import { invoke } from '@tauri-apps/api/core';
import type {
  AppSettings,
  GameLanguage,
  Instance,
  JavaRecommendation,
  JavaVersion,
  LaunchResult,
  MojangVersionInfo,
  ServerBookmark,
  ServerStatus,
} from './types';

// instances
export const getInstances = () => invoke<Instance[]>('get_instances');
export const createInstance = (args: {
  name: string;
  version: string;
  loader?: string;
  account_name?: string;
  language?: string;
}) => invoke<Instance>('create_instance', args);
export const updateInstance = (args: {
  id: string;
  changes: Record<string, unknown>;
}) => invoke<Instance>('update_instance', args);
export const setTags = (id: string, tags: string[]) =>
  invoke('set_instance_tags', { id, tags });
export const getAllTags = () => invoke<string[]>('get_all_tags');
export const deleteInstance = (id: string) => invoke('delete_instance', { id });
export const launchInstance = (id: string) =>
  invoke<LaunchResult>('launch_instance', { id });
export const readLogTail = (instanceId: string, lines: number = 300) =>
  invoke<string[]>('read_log_tail', { instanceId, lines });
export const buildLaunchArgs = (id: string) =>
  invoke<string>('build_launch_args', { id });
export const exportInstance = (id: string) =>
  invoke<Instance>('export_instance', { id });
export const importInstances = (instances: Instance[]) =>
  invoke<number>('import_instances', { instances });

// mojang
export const listMinecraftVersions = () =>
  invoke<MojangVersionInfo[]>('list_minecraft_versions');

// game language (142 种，含小众语言)
export const gameLanguages = () => invoke<GameLanguage[]>('game_languages');
export const matchGameLanguage = (systemLocale: string) =>
  invoke<GameLanguage | null>('match_game_language', { systemLocale });

// java
export const listJava = () => invoke<JavaVersion[]>('list_java');
export const installJava = (version: string) =>
  invoke<JavaVersion>('install_java', { version });
export const recommendJava = (mcVersion: string) =>
  invoke<JavaRecommendation>('recommend_java', { mcVersion });

// servers
export const listServers = () => invoke<ServerBookmark[]>('list_servers');
export const addServer = (address: string, name: string) =>
  invoke<ServerBookmark>('add_server', { address, name });
export const removeServer = (id: string) => invoke('remove_server', { id });
export const toggleServerFavorite = (id: string) =>
  invoke('toggle_server_favorite', { id });
export const queryServer = (address: string) =>
  invoke<ServerStatus>('query_server', { address });

// settings
export const getSettings = () => invoke<AppSettings>('get_settings');
export const saveSettings = (settings: AppSettings) =>
  invoke('save_settings', { settings });