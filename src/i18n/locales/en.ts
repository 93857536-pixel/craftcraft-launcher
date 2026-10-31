/**
 * English — source of truth for all translatable keys.
 * Add a new key here first, then mirror it into every other locale file.
 */
const en = {
  'app.name': 'CraftCraft',

  // navigation
  'nav.instances': 'Instances',
  'nav.newInstance': 'New Instance',
  'nav.java': 'Java',
  'nav.logs': 'Logs',
  'nav.servers': 'Servers',
  'nav.settings': 'Settings',

  // generic actions
  'common.cancel': 'Cancel',
  'common.create': 'Create',
  'common.save': 'Save',
  'common.delete': 'Delete',
  'common.del': 'Del',
  'common.edit': 'Edit',
  'common.launch': 'Launch',
  'common.all': 'All',
  'common.filter': 'Filter',
  'common.add': 'Add',
  'common.test': 'Test',
  'common.refresh': 'Refresh',
  'common.downloading': 'Downloading…',
  'common.optional': 'optional',

  // instances
  'instances.title': 'Instances',
  'instances.count_one': '{n} instance',
  'instances.count_other': '{n} instances',
  'instances.filteredBy': 'filtered: {tag}',
  'instances.empty.title': 'No instances yet',
  'instances.empty.desc': 'Create your first Minecraft instance to get started.',
  'instances.deleteConfirm': 'Delete this instance? Game data will be kept.',
  'instances.launchFailed': 'Launch failed: {msg}',
  'instances.addTag': 'Add tag…',
  'instances.newCard': 'New Instance',
  'instances.languageAuto': 'Game language: follow game',
  'instances.versionDesc': 'Change game version (applies next launch)',
  'instances.language': 'Game language',
  'instances.languageCommon': 'Common languages',
  'instances.languageMore': 'More languages (incl. Klingon, Quenya, toki pona…)',
  'instances.languageDesc': 'Minecraft in-game language — passed to the game as --language',

  // create-instance wizard
  'wizard.title': 'Create Instance',
  'wizard.name': 'Name',
  'wizard.namePlaceholder': 'My Survival World',
  'wizard.version': 'Version',
  'wizard.versionPlaceholder': '1.21',
  'wizard.selectVersion': 'Select a version…',
  'wizard.releases': 'Releases',
  'wizard.snapshots': 'Snapshots',
  'wizard.oldBeta': 'Old Beta',
  'wizard.oldAlpha': 'Old Alpha',
  'wizard.showSnapshots': 'Include snapshots ({n})',
  'wizard.notInManifest': 'Not in manifest — used as-is',
  'wizard.loader': 'Loader',
  'wizard.accountName': 'Account Name',
  'wizard.maxMemory': 'Max Memory (MB)',
  'wizard.javaRuntime': 'Java Runtime',
  'wizard.jdkAvailable': '{n} JDK(s) available',
  'wizard.noJdk': 'No JDK found — download one below',
  'wizard.createFailed': 'Create failed: {msg}',
  'wizard.create': 'Create Instance',

  // java runtime page
  'java.title': 'Java Runtime',
  'java.downloadTemurin': 'Download Temurin',
  'java.installed': 'Installed JDKs',
  'java.col.version': 'Version',
  'java.col.path': 'Path',
  'java.col.vendor': 'Vendor',
  'java.col.arch': 'Arch',
  'java.col.type': 'Type',
  'java.managed': 'Managed',
  'java.system': 'System',
  'java.empty': 'No JDKs found. Download one above.',
  'java.legacyNote': 'Minecraft 1.12 and older needs Java 8. On Apple Silicon install an x86_64 Java 8 build (runs via Rosetta 2).',

  // logs page
  'logs.title': 'Logs',
  'logs.noSelection': 'No instance selected',
  'logs.noSelectionDesc': 'Launch an instance to view its logs here.',
  'logs.createInstance': '+ Create Instance',
  'logs.selectInstance': 'Select {name}',
  'logs.autoRefresh': 'Auto-refresh',
  'logs.empty': 'No log yet. Launch the game to generate one.',

  // servers page
  'servers.title': 'Servers',
  'servers.addServer': 'Add Server',
  'servers.address': 'address:port',
  'servers.name': 'name (optional)',
  'servers.online': 'Online',
  'servers.offline': 'Offline',
  'servers.players': '{online}/{max} players',
  'servers.clickTest': 'Click Test to check',
  'servers.empty': 'No servers. Add one above.',

  // settings page
  'settings.title': 'Settings',
  'settings.defaultMaxMemory': 'Default Max Memory (MB)',
  'settings.defaultMinMemory': 'Default Min Memory (MB)',
  'settings.autoSwitchLogs': 'Auto-switch to Logs tab after launch',
  'settings.save': 'Save Settings',
  'settings.language': 'Language',
  'settings.languageDesc': 'Interface language — changes apply instantly',
  'settings.saved': 'Settings saved',
};

export default en;
export type Dict = Record<string, string>;
