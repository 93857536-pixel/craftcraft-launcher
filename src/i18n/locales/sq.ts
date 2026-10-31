/** Shqip */
const sq = {
  'app.name': 'CraftCraft',

  // navigation
  'nav.instances': 'Instancat',
  'nav.newInstance': 'Instancë e Re',
  'nav.java': 'Java',
  'nav.logs': 'Regjistrat',
  'nav.servers': 'Serverat',
  'nav.settings': 'Cilësimet',

  // generic actions
  'common.cancel': 'Anulo',
  'common.create': 'Krijo',
  'common.save': 'Ruaj',
  'common.delete': 'Fshi',
  'common.del': 'Fshij',
  'common.edit': 'Modifiko',
  'common.launch': 'Nis',
  'common.all': 'Të gjitha',
  'common.filter': 'Filtro',
  'common.add': 'Shto',
  'common.test': 'Testo',
  'common.refresh': 'Rifresko',
  'common.downloading': 'Duke shkarkuar…',
  'common.optional': 'opsionale',

  // instances
  'instances.title': 'Instancat',
  'instances.count_one': '{n} instancë',
  'instances.count_other': '{n} instanca',
  'instances.filteredBy': 'të filtruara: {tag}',
  'instances.empty.title': 'Nuk ka ende instanca',
  'instances.empty.desc': 'Krijo instancën tënde të parë të Minecraft për të filluar.',
  'instances.deleteConfirm': 'Të fshihet kjo instancë? Të dhënat e lojës do të ruhen.',
  'instances.launchFailed': 'Nisja dështoi: {msg}',
  'instances.addTag': 'Shto etiketë…',
  'instances.newCard': 'Instancë e Re',

  // create-instance wizard
  'wizard.title': 'Krijo Instancë',
  'wizard.name': 'Emri',
  'wizard.namePlaceholder': 'Bota Ime e Mbijetesës',
  'wizard.version': 'Versioni',
  'wizard.versionPlaceholder': '1.21',
  'wizard.notInManifest': 'Nuk është në manifest — përdoret si është',
  'wizard.loader': 'Ngarkuesi',
  'wizard.accountName': 'Emri i Llogarisë',
  'wizard.maxMemory': 'Memoria Maksimale (MB)',
  'wizard.javaRuntime': 'Runtime i Java',
  'wizard.jdkAvailable': '{n} JDK të disponueshme',
  'wizard.noJdk': 'Nuk u gjet asnjë JDK — shkarko një më poshtë',
  'wizard.createFailed': 'Krijimi dështoi: {msg}',
  'wizard.create': 'Krijo Instancë',

  // java runtime page
  'java.title': 'Runtime i Java',
  'java.downloadTemurin': 'Shkarko Temurin',
  'java.installed': 'JDK të Instaluara',
  'java.col.version': 'Versioni',
  'java.col.path': 'Rruga',
  'java.col.vendor': 'Furnizuesi',
  'java.col.arch': 'Arkitektura',
  'java.col.type': 'Tipi',
  'java.managed': 'E menaxhuar',
  'java.system': 'Sistemi',
  'java.empty': 'Nuk u gjet asnjë JDK. Shkarko një më sipër.',
  'java.legacyNote': 'Minecraft 1.12 dhe më i vjetër kërkon Java 8. Në Apple Silicon instaloni një ndërtim x86_64 të Java 8 (funksionon përmes Rosetta 2).',

  // logs page
  'logs.title': 'Regjistrat',
  'logs.noSelection': 'Nuk është zgjedhur asnjë instancë',
  'logs.noSelectionDesc': 'Nis një instancë për të parë regjistrat e saj këtu.',
  'logs.createInstance': '+ Krijo Instancë',
  'logs.selectInstance': 'Zgjidh {name}',
  'logs.autoRefresh': 'Rifreskim automatik',
  'logs.empty': 'Nuk ka ende regjistër. Nis lojën për të gjeneruar një.',

  // servers page
  'servers.title': 'Serverat',
  'servers.addServer': 'Shto Server',
  'servers.address': 'adresa:porta',
  'servers.name': 'emri (opsionale)',
  'servers.online': 'Në linjë',
  'servers.offline': 'Jashtë linje',
  'servers.players': '{online}/{max} lojtarë',
  'servers.clickTest': 'Kliko Testo për të kontrolluar',
  'servers.empty': 'Nuk ka servera. Shto një më sipër.',

  // settings page
  'settings.title': 'Cilësimet',
  'settings.defaultMaxMemory': 'Memoria Maksimale e Parazgjedhur (MB)',
  'settings.defaultMinMemory': 'Memoria Minimale e Parazgjedhur (MB)',
  'settings.autoSwitchLogs': 'Kaloni automatikisht te skeda Regjistrat pas nisjes',
  'settings.save': 'Ruaj Cilësimet',
  'settings.language': 'Gjuha',
  'settings.languageDesc': 'Gjuha e ndërfaqes — ndryshimet zbatohen menjëherë',
  'settings.saved': 'Cilësimet u ruajtën',
};

export default sq;
