/** Eesti */
const et = {
  'app.name': 'CraftCraft',

  // navigation
  'nav.instances': 'Eksemplarid',
  'nav.newInstance': 'Uus eksemplar',
  'nav.java': 'Java',
  'nav.logs': 'Logid',
  'nav.servers': 'Serverid',
  'nav.settings': 'Seaded',

  // generic actions
  'common.cancel': 'Tühista',
  'common.create': 'Loo',
  'common.save': 'Salvesta',
  'common.delete': 'Kustuta',
  'common.del': 'Kustuta',
  'common.edit': 'Muuda',
  'common.launch': 'Käivita',
  'common.all': 'Kõik',
  'common.filter': 'Filter',
  'common.add': 'Lisa',
  'common.test': 'Testi',
  'common.refresh': 'Värskenda',
  'common.downloading': 'Allalaadimine…',
  'common.optional': 'valikuline',

  // instances
  'instances.title': 'Eksemplarid',
  'instances.count_one': '{n} eksemplar',
  'instances.count_other': '{n} eksemplari',
  'instances.filteredBy': 'filtreeritud: {tag}',
  'instances.empty.title': 'Eksemplare veel pole',
  'instances.empty.desc': 'Alustamiseks loo oma esimene Minecrafti eksemplar.',
  'instances.deleteConfirm': 'Kas kustutada see eksemplar? Mänguandmed säilitatakse.',
  'instances.launchFailed': 'Käivitamine ebaõnnestus: {msg}',
  'instances.addTag': 'Lisa silt…',
  'instances.newCard': 'Uus eksemplar',

  'instances.languageAuto': 'Mängukeel: järgib mängu seadeid',
  'instances.language': 'Mängukeel',
  'instances.languageCommon': 'Tüüpilised keeled',
  'instances.languageMore': 'Rohkem keeli (sh. klingoni, quenya ja toki pona…)',
  'instances.languageDesc': 'Minecrafti mängusisene keel — edastatakse mängule parameetrina --language',
  // create-instance wizard
  'wizard.title': 'Eksemplari loomine',
  'wizard.name': 'Nimi',
  'wizard.namePlaceholder': 'Minu ellujäämismaailm',
  'wizard.version': 'Versioon',
  'wizard.versionPlaceholder': '1.21',
  'wizard.notInManifest': 'Manifestis pole — kasutatakse nii, nagu on',
  'wizard.loader': 'Laadija',
  'wizard.accountName': 'Konto nimi',
  'wizard.maxMemory': 'Maksimaalne mälu (MB)',
  'wizard.javaRuntime': 'Java käituskeskkond',
  'wizard.jdkAvailable': 'Saadaval JDK-sid: {n}',
  'wizard.noJdk': 'JDK-d ei leitud — laadi üks allpool',
  'wizard.createFailed': 'Loomine ebaõnnestus: {msg}',
  'wizard.create': 'Loo eksemplar',

  // java runtime page
  'java.title': 'Java käituskeskkond',
  'java.downloadTemurin': 'Laadi alla Temurin',
  'java.installed': 'Paigaldatud JDK-d',
  'java.col.version': 'Versioon',
  'java.col.path': 'Tee',
  'java.col.vendor': 'Tootja',
  'java.col.arch': 'Arhitektuur',
  'java.col.type': 'Tüüp',
  'java.managed': 'Hallatav',
  'java.system': 'Süsteemne',
  'java.empty': 'JDK-sid ei leitud. Laadi üks ülalt alla.',
  'java.legacyNote': 'Minecraft 1.12 ja vanemad versioonid vajavad Java 8-t. Apple Siliconil paigalda x86_64 Java 8 versioon (jookseb Rosetta 2 kaudu).',

  // logs page
  'logs.title': 'Logid',
  'logs.noSelection': 'Ühtegi eksemplari pole valitud',
  'logs.noSelectionDesc': 'Käivita eksemplar, et selle logi siin näha.',
  'logs.createInstance': '+ Loo eksemplar',
  'logs.selectInstance': 'Vali {name}',
  'logs.autoRefresh': 'Automaatne värskendamine',
  'logs.empty': 'Logi veel pole. Käivita mäng, et see luua.',

  // servers page
  'servers.title': 'Serverid',
  'servers.addServer': 'Lisa server',
  'servers.address': 'aadress:port',
  'servers.name': 'nimi (valikuline)',
  'servers.online': 'Võrgus',
  'servers.offline': 'Võrgust väljas',
  'servers.players': '{online}/{max} mängijat',
  'servers.clickTest': 'Kontrollimiseks klõpsa Testi',
  'servers.empty': 'Servereid pole. Lisa üks ülalt.',

  // settings page
  'settings.title': 'Seaded',
  'settings.defaultMaxMemory': 'Vaikimisi maksimaalne mälu (MB)',
  'settings.defaultMinMemory': 'Vaikimisi minimaalne mälu (MB)',
  'settings.autoSwitchLogs': 'Pärast käivitamist lülitu automaatselt vahekaardile Logid',
  'settings.save': 'Salvesta seaded',
  'settings.language': 'Keel',
  'settings.languageDesc': 'Kasutajaliidese keel — muudatused rakenduvad kohe',
  'settings.saved': 'Seaded salvestatud',
};

export default et;
