/** Magyar */
const hu = {
  'app.name': 'CraftCraft',

  // navigation
  'nav.instances': 'Példányok',
  'nav.newInstance': 'Új példány',
  'nav.java': 'Java',
  'nav.logs': 'Naplók',
  'nav.servers': 'Szerverek',
  'nav.settings': 'Beállítások',

  // generic actions
  'common.cancel': 'Mégse',
  'common.create': 'Létrehozás',
  'common.save': 'Mentés',
  'common.delete': 'Törlés',
  'common.del': 'Törl.',
  'common.edit': 'Szerkesztés',
  'common.launch': 'Indítás',
  'common.all': 'Összes',
  'common.filter': 'Szűrő',
  'common.add': 'Hozzáadás',
  'common.test': 'Teszt',
  'common.refresh': 'Frissítés',
  'common.downloading': 'Letöltés…',
  'common.optional': 'nem kötelező',

  // instances
  'instances.title': 'Példányok',
  'instances.count_one': '{n} példány',
  'instances.count_other': '{n} példány',
  'instances.filteredBy': 'szűrve: {tag}',
  'instances.empty.title': 'Még nincsenek példányok',
  'instances.empty.desc': 'Hozd létre az első Minecraft példányodat a kezdéshez.',
  'instances.deleteConfirm': 'Törlöd ezt a példányt? A játék adatai megmaradnak.',
  'instances.launchFailed': 'Az indítás sikertelen: {msg}',
  'instances.addTag': 'Címke hozzáadása…',
  'instances.newCard': 'Új példány',

  // create-instance wizard
  'wizard.title': 'Példány létrehozása',
  'wizard.name': 'Név',
  'wizard.namePlaceholder': 'Saját túlélő világom',
  'wizard.version': 'Verzió',
  'wizard.versionPlaceholder': '1.21',
  'wizard.notInManifest': 'Nincs a jegyzékben — változatlanul használva',
  'wizard.loader': 'Betöltő',
  'wizard.accountName': 'Fióknév',
  'wizard.maxMemory': 'Maximális memória (MB)',
  'wizard.javaRuntime': 'Java futtatókörnyezet',
  'wizard.jdkAvailable': '{n} JDK érhető el',
  'wizard.noJdk': 'Nem található JDK — tölts le egyet alább',
  'wizard.createFailed': 'A létrehozás sikertelen: {msg}',
  'wizard.create': 'Példány létrehozása',

  // java runtime page
  'java.title': 'Java futtatókörnyezet',
  'java.downloadTemurin': 'Temurin letöltése',
  'java.installed': 'Telepített JDK-k',
  'java.col.version': 'Verzió',
  'java.col.path': 'Útvonal',
  'java.col.vendor': 'Gyártó',
  'java.col.arch': 'Architektúra',
  'java.col.type': 'Típus',
  'java.managed': 'Kezelt',
  'java.system': 'Rendszer',
  'java.empty': 'Nem található JDK. Tölts le egyet fentebb.',
  'java.legacyNote': 'A Minecraft 1.12 és a régebbi verziók Java 8-at igényelnek. Apple Silicon esetén telepíts x86_64-es Java 8-at (Rosetta 2-n fut).',

  // logs page
  'logs.title': 'Naplók',
  'logs.noSelection': 'Nincs kiválasztott példány',
  'logs.noSelectionDesc': 'Indíts el egy példányt, hogy itt lásd a naplóját.',
  'logs.createInstance': '+ Példány létrehozása',
  'logs.selectInstance': '{name} kiválasztása',
  'logs.autoRefresh': 'Automatikus frissítés',
  'logs.empty': 'Még nincs napló. Indítsd el a játékot, hogy létrejöjjön.',

  // servers page
  'servers.title': 'Szerverek',
  'servers.addServer': 'Szerver hozzáadása',
  'servers.address': 'cím:port',
  'servers.name': 'név (nem kötelező)',
  'servers.online': 'Online',
  'servers.offline': 'Offline',
  'servers.players': '{online}/{max} játékos',
  'servers.clickTest': 'Kattints a Teszt gombra az ellenőrzéshez',
  'servers.empty': 'Nincsenek szerverek. Adj hozzá egyet fentebb.',

  // settings page
  'settings.title': 'Beállítások',
  'settings.defaultMaxMemory': 'Alapértelmezett maximális memória (MB)',
  'settings.defaultMinMemory': 'Alapértelmezett minimális memória (MB)',
  'settings.autoSwitchLogs': 'Indítás után automatikus váltás a Naplók fülére',
  'settings.save': 'Beállítások mentése',
  'settings.language': 'Nyelv',
  'settings.languageDesc': 'A felület nyelve — a változtatások azonnal életbe lépnek',
  'settings.saved': 'Beállítások mentve',
};

export default hu;
