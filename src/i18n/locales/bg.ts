/** Български */
const bg = {
  'app.name': 'CraftCraft',

  // navigation
  'nav.instances': 'Инстанции',
  'nav.newInstance': 'Нова инстанция',
  'nav.java': 'Java',
  'nav.logs': 'Дневници',
  'nav.servers': 'Сървъри',
  'nav.settings': 'Настройки',

  // generic actions
  'common.cancel': 'Отказ',
  'common.create': 'Създай',
  'common.save': 'Запази',
  'common.delete': 'Изтрий',
  'common.del': 'Изтр.',
  'common.edit': 'Редактирай',
  'common.launch': 'Стартирай',
  'common.all': 'Всички',
  'common.filter': 'Филтър',
  'common.add': 'Добави',
  'common.test': 'Тест',
  'common.refresh': 'Опресни',
  'common.downloading': 'Изтегляне…',
  'common.optional': 'по избор',

  // instances
  'instances.title': 'Инстанции',
  'instances.count_one': '{n} инстанция',
  'instances.count_other': '{n} инстанции',
  'instances.filteredBy': 'филтрирано: {tag}',
  'instances.empty.title': 'Все още няма инстанции',
  'instances.empty.desc': 'Създайте първата си Minecraft инстанция, за да започнете.',
  'instances.deleteConfirm': 'Да се изтрие ли тази инстанция? Данните от играта ще бъдат запазени.',
  'instances.launchFailed': 'Стартирането се провали: {msg}',
  'instances.addTag': 'Добави етикет…',
  'instances.newCard': 'Нова инстанция',

  'instances.languageAuto': 'Език на играта: следва настройките на играта',
  'instances.language': 'Език на играта',
  'instances.languageCommon': 'Често използвани езици',
  'instances.languageMore': 'Повече езици (вкл. клингонски, квеня и toki pona…)',
  'instances.languageDesc': 'Език в играта Minecraft — подава се на играта като параметър --language',
  // create-instance wizard
  'wizard.title': 'Създаване на инстанция',
  'wizard.name': 'Име',
  'wizard.namePlaceholder': 'Моят свят за оцеляване',
  'wizard.version': 'Версия',
  'wizard.versionPlaceholder': '1.21',
  'wizard.notInManifest': 'Няма го в манифеста — използва се както е',
  'wizard.loader': 'Зареждащ модул',
  'wizard.accountName': 'Име на профила',
  'wizard.maxMemory': 'Максимална памет (MB)',
  'wizard.javaRuntime': 'Java среда',
  'wizard.jdkAvailable': 'Налични JDK: {n}',
  'wizard.noJdk': 'Не е намерен JDK — изтеглете такъв по-долу',
  'wizard.createFailed': 'Създаването се провали: {msg}',
  'wizard.create': 'Създай инстанция',

  // java runtime page
  'java.title': 'Java среда',
  'java.downloadTemurin': 'Изтегли Temurin',
  'java.installed': 'Инсталирани JDK',
  'java.col.version': 'Версия',
  'java.col.path': 'Път',
  'java.col.vendor': 'Доставчик',
  'java.col.arch': 'Архитектура',
  'java.col.type': 'Тип',
  'java.managed': 'Управляван',
  'java.system': 'Системен',
  'java.empty': 'Не са намерени JDK. Изтеглете такъв по-горе.',
  'java.legacyNote': 'Minecraft 1.12 и по-старите версии изискват Java 8. На Apple Silicon инсталирайте x86_64 версия на Java 8 (работи чрез Rosetta 2).',

  // logs page
  'logs.title': 'Дневници',
  'logs.noSelection': 'Не е избрана инстанция',
  'logs.noSelectionDesc': 'Стартирайте инстанция, за да видите дневника ѝ тук.',
  'logs.createInstance': '+ Създай инстанция',
  'logs.selectInstance': 'Избери {name}',
  'logs.autoRefresh': 'Автоматично опресняване',
  'logs.empty': 'Все още няма дневник. Стартирайте играта, за да се създаде такъв.',

  // servers page
  'servers.title': 'Сървъри',
  'servers.addServer': 'Добави сървър',
  'servers.address': 'адрес:порт',
  'servers.name': 'име (по избор)',
  'servers.online': 'Онлайн',
  'servers.offline': 'Офлайн',
  'servers.players': '{online}/{max} играчи',
  'servers.clickTest': 'Натиснете Тест, за да проверите',
  'servers.empty': 'Няма сървъри. Добавете един по-горе.',

  // settings page
  'settings.title': 'Настройки',
  'settings.defaultMaxMemory': 'Максимална памет по подразбиране (MB)',
  'settings.defaultMinMemory': 'Минимална памет по подразбиране (MB)',
  'settings.autoSwitchLogs': 'Автоматично превключване към раздела Дневници след стартиране',
  'settings.save': 'Запази настройките',
  'settings.language': 'Език',
  'settings.languageDesc': 'Език на интерфейса — промените се прилагат веднага',
  'settings.saved': 'Настройките са запазени',
};

export default bg;
