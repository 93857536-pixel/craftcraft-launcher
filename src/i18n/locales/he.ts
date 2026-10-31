/** עברית */
const he = {
  'app.name': 'CraftCraft',

  // navigation
  'nav.instances': 'מופעים',
  'nav.newInstance': 'מופע חדש',
  'nav.java': 'Java',
  'nav.logs': 'יומנים',
  'nav.servers': 'שרתים',
  'nav.settings': 'הגדרות',

  // generic actions
  'common.cancel': 'ביטול',
  'common.create': 'יצירה',
  'common.save': 'שמירה',
  'common.delete': 'מחיקה',
  'common.del': 'מחק',
  'common.edit': 'עריכה',
  'common.launch': 'הפעלה',
  'common.all': 'הכול',
  'common.filter': 'סינון',
  'common.add': 'הוספה',
  'common.test': 'בדיקה',
  'common.refresh': 'רענון',
  'common.downloading': 'מוריד…',
  'common.optional': 'אופציונלי',

  // instances
  'instances.title': 'מופעים',
  'instances.count_one': 'מופע {n}',
  'instances.count_other': '{n} מופעים',
  'instances.filteredBy': 'מסונן: {tag}',
  'instances.empty.title': 'אין מופעים עדיין',
  'instances.empty.desc': 'צור את מופע ה-Minecraft הראשון שלך כדי להתחיל.',
  'instances.deleteConfirm': 'למחוק את המופע הזה? נתוני המשחק יישמרו.',
  'instances.launchFailed': 'ההפעלה נכשלה: {msg}',
  'instances.addTag': 'הוספת תגית…',
  'instances.newCard': 'מופע חדש',
  'instances.languageAuto': 'שפת המשחק: עוקבת את הגדרות המשחק',
  'instances.language': 'שפת המשחק',
  'instances.languageCommon': 'שפות נפוצות',
  'instances.languageMore': 'שפות נוספות (כולל Klingon, Quenya, toki pona…)',
  'instances.languageDesc': 'שפת המשחק של Minecraft — מועברת למשחק באמצעות --language',

  // create-instance wizard
  'wizard.title': 'יצירת מופע',
  'wizard.name': 'שם',
  'wizard.namePlaceholder': 'עולם ההישרדות שלי',
  'wizard.version': 'גרסה',
  'wizard.versionPlaceholder': '1.21',
  'wizard.notInManifest': 'לא ברשימה — ישמש כפי שהוא',
  'wizard.loader': 'מטעין',
  'wizard.accountName': 'שם חשבון',
  'wizard.maxMemory': 'זיכרון מקסימלי (MB)',
  'wizard.javaRuntime': 'סביבת Java',
  'wizard.jdkAvailable': '{n} חבילות JDK זמינות',
  'wizard.noJdk': 'לא נמצא JDK — הורד אחד למטה',
  'wizard.createFailed': 'היצירה נכשלה: {msg}',
  'wizard.create': 'יצירת מופע',

  // java runtime page
  'java.title': 'סביבת Java',
  'java.downloadTemurin': 'הורדת Temurin',
  'java.installed': 'חבילות JDK מותקנות',
  'java.col.version': 'גרסה',
  'java.col.path': 'נתיב',
  'java.col.vendor': 'ספק',
  'java.col.arch': 'ארכיטקטורה',
  'java.col.type': 'סוג',
  'java.managed': 'מנוהל',
  'java.system': 'מערכת',
  'java.empty': 'לא נמצאו חבילות JDK. הורד אחת למעלה.',
  'java.legacyNote': 'Minecraft 1.12 ומטה דורש Java 8. ב-Apple Silicon התקן גרסת Java 8 עבור x86_64 (פועלת דרך Rosetta 2).',

  // logs page
  'logs.title': 'יומנים',
  'logs.noSelection': 'לא נבחר מופע',
  'logs.noSelectionDesc': 'הפעל מופע כדי לצפות ביומנים שלו כאן.',
  'logs.createInstance': '+ יצירת מופע',
  'logs.selectInstance': 'בחר {name}',
  'logs.autoRefresh': 'רענון אוטומטי',
  'logs.empty': 'אין יומן עדיין. הפעל את המשחק כדי ליצור אחד.',

  // servers page
  'servers.title': 'שרתים',
  'servers.addServer': 'הוספת שרת',
  'servers.address': 'כתובת:פורט',
  'servers.name': 'שם (אופציונלי)',
  'servers.online': 'מקוון',
  'servers.offline': 'לא מקוון',
  'servers.players': '{online}/{max} שחקנים',
  'servers.clickTest': 'לחץ בדיקה כדי לבדוק',
  'servers.empty': 'אין שרתים. הוסף אחד למעלה.',

  // settings page
  'settings.title': 'הגדרות',
  'settings.defaultMaxMemory': 'זיכרון מקסימלי ברירת מחדל (MB)',
  'settings.defaultMinMemory': 'זיכרון מינימלי ברירת מחדל (MB)',
  'settings.autoSwitchLogs': 'מעבר אוטומטי ללשונית היומנים לאחר ההפעלה',
  'settings.save': 'שמירת הגדרות',
  'settings.language': 'שפה',
  'settings.languageDesc': 'שפת הממשק — השינויים חלים מיד',
  'settings.saved': 'ההגדרות נשמרו',
};

export default he;
