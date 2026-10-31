/** Lietuvių */
const lt = {
  'app.name': 'CraftCraft',

  // navigation
  'nav.instances': 'Egzemplioriai',
  'nav.newInstance': 'Naujas egzempliorius',
  'nav.java': 'Java',
  'nav.logs': 'Žurnalai',
  'nav.servers': 'Serveriai',
  'nav.settings': 'Nustatymai',

  // generic actions
  'common.cancel': 'Atšaukti',
  'common.create': 'Sukurti',
  'common.save': 'Išsaugoti',
  'common.delete': 'Ištrinti',
  'common.del': 'Trinti',
  'common.edit': 'Redaguoti',
  'common.launch': 'Paleisti',
  'common.all': 'Visi',
  'common.filter': 'Filtras',
  'common.add': 'Pridėti',
  'common.test': 'Tikrinti',
  'common.refresh': 'Atnaujinti',
  'common.downloading': 'Atsiunčiama…',
  'common.optional': 'neprivaloma',

  // instances
  'instances.title': 'Egzemplioriai',
  'instances.count_one': '{n} egzempliorius',
  'instances.count_other': '{n} egzempliorių',
  'instances.filteredBy': 'filtruota: {tag}',
  'instances.empty.title': 'Egzempliorių dar nėra',
  'instances.empty.desc': 'Sukurkite pirmąjį Minecraft egzempliorių, kad pradėtumėte.',
  'instances.deleteConfirm': 'Ištrinti šį egzempliorių? Žaidimo duomenys bus išsaugoti.',
  'instances.launchFailed': 'Paleisti nepavyko: {msg}',
  'instances.addTag': 'Pridėti žymą…',
  'instances.newCard': 'Naujas egzempliorius',
  'instances.languageAuto': 'Žaidimo kalba: seka žaidimo nustatymus',
  'instances.language': 'Žaidimo kalba',
  'instances.languageCommon': 'Populiarios kalbos',
  'instances.languageMore': 'Daugiau kalbų (įskaitant Klingon, Quenya, toki pona…)',
  'instances.languageDesc': 'Minecraft kalba žaismui — perduodama žaidimui kaip --language',

  // create-instance wizard
  'wizard.title': 'Egzemplioriaus kūrimas',
  'wizard.name': 'Pavadinimas',
  'wizard.namePlaceholder': 'Mano išgyvenimo pasaulis',
  'wizard.version': 'Versija',
  'wizard.versionPlaceholder': '1.21',
  'wizard.notInManifest': 'Nėra manifeste — naudojama tokia, kokia yra',
  'wizard.loader': 'Įkėlėjas',
  'wizard.accountName': 'Paskyros pavadinimas',
  'wizard.maxMemory': 'Didžiausia atmintis (MB)',
  'wizard.javaRuntime': 'Java vykdymo aplinka',
  'wizard.jdkAvailable': 'Pasiekiama JDK: {n}',
  'wizard.noJdk': 'JDK nerasta — atsisiųskite vieną žemiau',
  'wizard.createFailed': 'Sukurti nepavyko: {msg}',
  'wizard.create': 'Sukurti egzempliorių',

  // java runtime page
  'java.title': 'Java vykdymo aplinka',
  'java.downloadTemurin': 'Atsisiųsti Temurin',
  'java.installed': 'Įdiegtos JDK',
  'java.col.version': 'Versija',
  'java.col.path': 'Kelias',
  'java.col.vendor': 'Tiekėjas',
  'java.col.arch': 'Architektūra',
  'java.col.type': 'Tipas',
  'java.managed': 'Valdoma',
  'java.system': 'Sistemos',
  'java.empty': 'JDK nerasta. Atsisiųskite vieną aukščiau.',
  'java.legacyNote': 'Minecraft 1.12 ir senesnėms versijoms reikia Java 8. „Apple Silicon“ įdiekite x86_64 Java 8 versiją (veikia per Rosetta 2).',

  // logs page
  'logs.title': 'Žurnalai',
  'logs.noSelection': 'Nepasirinktas egzempliorius',
  'logs.noSelectionDesc': 'Paleiskite egzempliorių, kad čia matytumėte jo žurnalą.',
  'logs.createInstance': '+ Sukurti egzempliorių',
  'logs.selectInstance': 'Pasirinkti {name}',
  'logs.autoRefresh': 'Automatinis atnaujinimas',
  'logs.empty': 'Žurnalo dar nėra. Paleiskite žaidimą, kad jis būtų sukurtas.',

  // servers page
  'servers.title': 'Serveriai',
  'servers.addServer': 'Pridėti serverį',
  'servers.address': 'adresas:portas',
  'servers.name': 'pavadinimas (neprivaloma)',
  'servers.online': 'Prisijungta',
  'servers.offline': 'Neprisijungta',
  'servers.players': '{online}/{max} žaidėjų',
  'servers.clickTest': 'Spustelėkite Tikrinti, kad patikrintumėte',
  'servers.empty': 'Nėra serverių. Pridėkite vieną aukščiau.',

  // settings page
  'settings.title': 'Nustatymai',
  'settings.defaultMaxMemory': 'Numatytoji didžiausia atmintis (MB)',
  'settings.defaultMinMemory': 'Numatytoji mažiausia atmintis (MB)',
  'settings.autoSwitchLogs': 'Po paleidimo automatiškai perjungti į skirtuką Žurnalai',
  'settings.save': 'Išsaugoti nustatymus',
  'settings.language': 'Kalba',
  'settings.languageDesc': 'Sąsajos kalba — pakeitimai taikomi iš karto',
  'settings.saved': 'Nustatymai išsaugoti',
};

export default lt;
