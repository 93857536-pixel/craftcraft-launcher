export type LoaderType = 'vanilla' | 'forge' | 'fabric' | 'quilt' | 'neoforge';

export interface Instance {
  id: string;
  name: string;
  version: string;
  loader: LoaderType;
  loader_version: string | null;
  java_version: string | null;
  java_path: string | null;
  game_directory: string;
  memory_min_mb: number;
  memory_max_mb: number;
  jvm_extra_args: string[];
  account_name: string;
  language: string | null;
  last_played_at: number | null;
  play_count: number;
  created_at: number;
  tags: string[];
}

export interface LaunchResult {
  success: boolean;
  pid: number | null;
  message: string;
}

/** 一条 Minecraft 游戏内语言（后端内置 142 种，含克林贡语等小众语言）。 */
export interface GameLanguage {
  code: string;
  name: string;
  region: string;
}

export interface JavaVersion {
  id: string;
  path: string;
  version: string;
  vendor: string;
  architecture: string;
  is_managed: boolean;
}

export interface MojangVersionInfo {
  id: string;
  type: string | null;
  url: string | null;
}

export interface ServerBookmark {
  id: string;
  address: string;
  name: string;
  description: string | null;
  is_favorite: boolean;
  last_accessed_at: number | null;
  access_count: number;
}

export interface ServerStatus {
  reachable: boolean;
  motd: string | null;
  online: number | null;
  max: number | null;
  version: string | null;
  error: string | null;
}

export interface AppSettings {
  theme: string;
  java_home_override: string | null;
  default_memory_min_mb: number;
  default_memory_max_mb: number;
  open_log_on_launch: boolean;
}

export interface JavaRecommendation {
  mc_version: string;
  recommended_java: string;
  minimum_java: string;
  explanation: string;
}
