/** Latviešu */
const lv = {
  'app.name': 'CraftCraft',

  // navigation
  'nav.instances': 'Instances',
  'nav.newInstance': 'Jauna instance',
  'nav.java': 'Java',
  'nav.logs': 'Žurnāli',
  'nav.servers': 'Serveri',
  'nav.settings': 'Iestatījumi',

  // generic actions
  'common.cancel': 'Atcelt',
  'common.create': 'Izveidot',
  'common.save': 'Saglabāt',
  'common.delete': 'Dzēst',
  'common.del': 'Dzēst',
  'common.edit': 'Rediģēt',
  'common.launch': 'Palaist',
  'common.all': 'Visi',
  'common.filter': 'Filtrs',
  'common.add': 'Pievienot',
  'common.test': 'Pārbaudīt',
  'common.refresh': 'Atjaunot',
  'common.downloading': 'Lejupielādē…',
  'common.optional': 'neobligāti',

  // instances
  'instances.title': 'Instances',
  'instances.count_one': '{n} instance',
  'instances.count_other': '{n} instances',
  'instances.filteredBy': 'filtrēts: {tag}',
  'instances.empty.title': 'Vēl nav nevienas instances',
  'instances.empty.desc': 'Izveidojiet savu pirmo Minecraft instanci, lai sāktu.',
  'instances.deleteConfirm': 'Dzēst šo instanci? Spēles dati tiks saglabāti.',
  'instances.launchFailed': 'Palaišana neizdevās: {msg}',
  'instances.addTag': 'Pievienot atzīmi…',
  'instances.newCard': 'Jauna instance',

  // create-instance wizard
  'wizard.title': 'Instances izveide',
  'wizard.name': 'Nosaukums',
  'wizard.namePlaceholder': 'Mana izdzīvošanas pasaule',
  'wizard.version': 'Versija',
  'wizard.versionPlaceholder': '1.21',
  'wizard.notInManifest': 'Nav manifestā — tiek izmantots tāds, kāds ir',
  'wizard.loader': 'Ielādētājs',
  'wizard.accountName': 'Konta nosaukums',
  'wizard.maxMemory': 'Maksimālā atmiņa (MB)',
  'wizard.javaRuntime': 'Java izpildlaiks',
  'wizard.jdkAvailable': 'Pieejami JDK: {n}',
  'wizard.noJdk': 'JDK nav atrasts — lejupielādējiet to zemāk',
  'wizard.createFailed': 'Izveide neizdevās: {msg}',
  'wizard.create': 'Izveidot instanci',

  // java runtime page
  'java.title': 'Java izpildlaiks',
  'java.downloadTemurin': 'Lejupielādēt Temurin',
  'java.installed': 'Instalētie JDK',
  'java.col.version': 'Versija',
  'java.col.path': 'Ceļš',
  'java.col.vendor': 'Ražotājs',
  'java.col.arch': 'Arhitektūra',
  'java.col.type': 'Tips',
  'java.managed': 'Pārvaldīts',
  'java.system': 'Sistēmas',
  'java.empty': 'JDK nav atrasts. Lejupielādējiet to augstāk.',
  'java.legacyNote': 'Minecraft 1.12 un vecākām versijām nepieciešama Java 8. Apple Silicon datoros instalējiet x86_64 Java 8 versiju (darbojas ar Rosetta 2).',

  // logs page
  'logs.title': 'Žurnāli',
  'logs.noSelection': 'Nav atlasīta instance',
  'logs.noSelectionDesc': 'Palaidiet instanci, lai šeit skatītu tās žurnālu.',
  'logs.createInstance': '+ Izveidot instanci',
  'logs.selectInstance': 'Atlasīt {name}',
  'logs.autoRefresh': 'Automātiska atjaunošana',
  'logs.empty': 'Žurnāla vēl nav. Palaidiet spēli, lai to izveidotu.',

  // servers page
  'servers.title': 'Serveri',
  'servers.addServer': 'Pievienot serveri',
  'servers.address': 'adrese:ports',
  'servers.name': 'nosaukums (neobligāts)',
  'servers.online': 'Tiešsaistē',
  'servers.offline': 'Bezsaistē',
  'servers.players': '{online}/{max} spēlētāji',
  'servers.clickTest': 'Noklikšķiniet uz Pārbaudīt, lai pārbaudītu',
  'servers.empty': 'Nav serveru. Pievienojiet vienu augstāk.',

  // settings page
  'settings.title': 'Iestatījumi',
  'settings.defaultMaxMemory': 'Noklusējuma maksimālā atmiņa (MB)',
  'settings.defaultMinMemory': 'Noklusējuma minimālā atmiņa (MB)',
  'settings.autoSwitchLogs': 'Pēc palaišanas automātiski pārslēgties uz cilni Žurnāli',
  'settings.save': 'Saglabāt iestatījumus',
  'settings.language': 'Valoda',
  'settings.languageDesc': 'Saskarnes valoda — izmaiņas stājas spēkā nekavējoties',
  'settings.saved': 'Iestatījumi saglabāti',
};

export default lv;
