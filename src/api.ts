import { invoke } from '@tauri-apps/api/core';
import type {
  AppSettings,
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
}) => invoke<Instance>('create_instance', args);
export const updateInstance = (args: {
  id: string;
  changes: Record<string, unknown>;
}) => invoke<Instance>('update_instance', args);
export const deleteInstance = (id: string) => invoke('delete_instance', { id });
export const launchInstance = (id: string) =>
  invoke<LaunchResult>('launch_instance', { id });
export const readLogTail = (instanceId: string, lines: number = 300) =>
  invoke<string[]>('read_log_tail', { instanceId, lines });

// mojang
export const listMinecraftVersions = () =>
  invoke<MojangVersionInfo[]>('list_minecraft_versions');

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
