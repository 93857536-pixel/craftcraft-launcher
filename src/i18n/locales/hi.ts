/** हिन्दी */
const hi = {
  'app.name': 'CraftCraft',

  // navigation
  'nav.instances': 'इंस्टेंस',
  'nav.newInstance': 'नया इंस्टेंस',
  'nav.java': 'Java',
  'nav.logs': 'लॉग',
  'nav.servers': 'सर्वर',
  'nav.settings': 'सेटिंग्स',

  // generic actions
  'common.cancel': 'रद्द करें',
  'common.create': 'बनाएँ',
  'common.save': 'सहेजें',
  'common.delete': 'हटाएँ',
  'common.del': 'हटाएँ',
  'common.edit': 'संपादित करें',
  'common.launch': 'लॉन्च करें',
  'common.all': 'सभी',
  'common.filter': 'फ़िल्टर',
  'common.add': 'जोड़ें',
  'common.test': 'जाँच',
  'common.refresh': 'रीफ़्रेश',
  'common.downloading': 'डाउनलोड हो रहा है…',
  'common.optional': 'वैकल्पिक',

  // instances
  'instances.title': 'इंस्टेंस',
  'instances.count_one': '{n} इंस्टेंस',
  'instances.count_other': '{n} इंस्टेंस',
  'instances.filteredBy': 'फ़िल्टर किया गया: {tag}',
  'instances.empty.title': 'अभी कोई इंस्टेंस नहीं',
  'instances.empty.desc': 'शुरू करने के लिए अपना पहला Minecraft इंस्टेंस बनाएँ।',
  'instances.deleteConfirm': 'यह इंस्टेंस हटाएँ? गेम डेटा सुरक्षित रहेगा।',
  'instances.launchFailed': 'लॉन्च विफल: {msg}',
  'instances.addTag': 'टैग जोड़ें…',
  'instances.newCard': 'नया इंस्टेंस',
  'instances.languageAuto': 'खेल की भाषा: खेल की सेटिंग्स के अनुसार',
  'instances.language': 'खेल की भाषा',
  'instances.languageCommon': 'सामान्य भाषाएँ',
  'instances.languageMore': 'अधिक भाषाएँ (Klingon, Quenya, toki pona सहित…)',
  'instances.languageDesc': 'Minecraft की इन-गेम भाषा — खेल को --language पैरामीटर के रूप में भेजी जाती है',

  // create-instance wizard
  'wizard.title': 'इंस्टेंस बनाएँ',
  'wizard.name': 'नाम',
  'wizard.namePlaceholder': 'मेरी सर्वाइवल दुनिया',
  'wizard.version': 'संस्करण',
  'wizard.versionPlaceholder': '1.21',
  'wizard.notInManifest': 'मैनिफ़ेस्ट में नहीं — जैसा है वैसा उपयोग होगा',
  'wizard.loader': 'लोडर',
  'wizard.accountName': 'खाता नाम',
  'wizard.maxMemory': 'अधिकतम मेमोरी (MB)',
  'wizard.javaRuntime': 'Java रनटाइम',
  'wizard.jdkAvailable': '{n} JDK उपलब्ध',
  'wizard.noJdk': 'कोई JDK नहीं मिला — नीचे से डाउनलोड करें',
  'wizard.createFailed': 'बनाना विफल: {msg}',
  'wizard.create': 'इंस्टेंस बनाएँ',

  // java runtime page
  'java.title': 'Java रनटाइम',
  'java.downloadTemurin': 'Temurin डाउनलोड करें',
  'java.installed': 'इंस्टॉल किए गए JDK',
  'java.col.version': 'संस्करण',
  'java.col.path': 'पथ',
  'java.col.vendor': 'विक्रेता',
  'java.col.arch': 'आर्क',
  'java.col.type': 'प्रकार',
  'java.managed': 'प्रबंधित',
  'java.system': 'सिस्टम',
  'java.empty': 'कोई JDK नहीं मिला। ऊपर से एक डाउनलोड करें।',
  'java.legacyNote': 'Minecraft 1.12 और पुराने के लिए Java 8 चाहिए। Apple Silicon पर x86_64 Java 8 बिल्ड इंस्टॉल करें (Rosetta 2 से चलता है)।',

  // logs page
  'logs.title': 'लॉग',
  'logs.noSelection': 'कोई इंस्टेंस चयनित नहीं',
  'logs.noSelectionDesc': 'लॉग देखने के लिए कोई इंस्टेंस लॉन्च करें।',
  'logs.createInstance': '+ इंस्टेंस बनाएँ',
  'logs.selectInstance': '{name} चुनें',
  'logs.autoRefresh': 'स्वतः रीफ़्रेश',
  'logs.empty': 'अभी कोई लॉग नहीं। लॉग बनाने के लिए गेम लॉन्च करें।',

  // servers page
  'servers.title': 'सर्वर',
  'servers.addServer': 'सर्वर जोड़ें',
  'servers.address': 'पता:पोर्ट',
  'servers.name': 'नाम (वैकल्पिक)',
  'servers.online': 'ऑनलाइन',
  'servers.offline': 'ऑफ़लाइन',
  'servers.players': '{online}/{max} खिलाड़ी',
  'servers.clickTest': 'जाँचने के लिए जाँच पर क्लिक करें',
  'servers.empty': 'कोई सर्वर नहीं। ऊपर एक जोड़ें।',

  // settings page
  'settings.title': 'सेटिंग्स',
  'settings.defaultMaxMemory': 'डिफ़ॉल्ट अधिकतम मेमोरी (MB)',
  'settings.defaultMinMemory': 'डिफ़ॉल्ट न्यूनतम मेमोरी (MB)',
  'settings.autoSwitchLogs': 'लॉन्च के बाद स्वतः लॉग टैब पर जाएँ',
  'settings.save': 'सेटिंग्स सहेजें',
  'settings.language': 'भाषा',
  'settings.languageDesc': 'इंटरफ़ेस भाषा — बदलाव तुरंत लागू होते हैं',
  'settings.saved': 'सेटिंग्स सहेजी गईं',
};

export default hi;
