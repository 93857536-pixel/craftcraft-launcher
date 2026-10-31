/** ไทย */
const th = {
  'app.name': 'CraftCraft',

  // navigation
  'nav.instances': 'อินสแตนซ์',
  'nav.newInstance': 'อินสแตนซ์ใหม่',
  'nav.java': 'Java',
  'nav.logs': 'บันทึก',
  'nav.servers': 'เซิร์ฟเวอร์',
  'nav.settings': 'การตั้งค่า',

  // generic actions
  'common.cancel': 'ยกเลิก',
  'common.create': 'สร้าง',
  'common.save': 'บันทึก',
  'common.delete': 'ลบ',
  'common.del': 'ลบ',
  'common.edit': 'แก้ไข',
  'common.launch': 'เปิดเกม',
  'common.all': 'ทั้งหมด',
  'common.filter': 'กรอง',
  'common.add': 'เพิ่ม',
  'common.test': 'ทดสอบ',
  'common.refresh': 'รีเฟรช',
  'common.downloading': 'กำลังดาวน์โหลด…',
  'common.optional': 'ไม่บังคับ',

  // instances
  'instances.title': 'อินสแตนซ์',
  'instances.count_one': '{n} อินสแตนซ์',
  'instances.count_other': '{n} อินสแตนซ์',
  'instances.filteredBy': 'กรองแล้ว: {tag}',
  'instances.empty.title': 'ยังไม่มีอินสแตนซ์',
  'instances.empty.desc': 'สร้างอินสแตนซ์ Minecraft แรกของคุณเพื่อเริ่มต้น',
  'instances.deleteConfirm': 'ลบอินสแตนซ์นี้หรือไม่? ข้อมูลเกมจะถูกเก็บไว้',
  'instances.launchFailed': 'เปิดไม่สำเร็จ: {msg}',
  'instances.addTag': 'เพิ่มแท็ก…',
  'instances.newCard': 'อินสแตนซ์ใหม่',

  // create-instance wizard
  'wizard.title': 'สร้างอินสแตนซ์',
  'wizard.name': 'ชื่อ',
  'wizard.namePlaceholder': 'โลกเซอร์ไววัลของฉัน',
  'wizard.version': 'เวอร์ชัน',
  'wizard.versionPlaceholder': '1.21',
  'wizard.notInManifest': 'ไม่อยู่ในรายการ — ใช้ตามที่ระบุ',
  'wizard.loader': 'ตัวโหลด',
  'wizard.accountName': 'ชื่อบัญชี',
  'wizard.maxMemory': 'หน่วยความจำสูงสุด (MB)',
  'wizard.javaRuntime': 'รันไทม์ Java',
  'wizard.jdkAvailable': 'มี JDK {n} รายการ',
  'wizard.noJdk': 'ไม่พบ JDK — ดาวน์โหลดด้านล่าง',
  'wizard.createFailed': 'สร้างไม่สำเร็จ: {msg}',
  'wizard.create': 'สร้างอินสแตนซ์',

  // java runtime page
  'java.title': 'รันไทม์ Java',
  'java.downloadTemurin': 'ดาวน์โหลด Temurin',
  'java.installed': 'JDK ที่ติดตั้งแล้ว',
  'java.col.version': 'เวอร์ชัน',
  'java.col.path': 'พาธ',
  'java.col.vendor': 'ผู้จำหน่าย',
  'java.col.arch': 'สถาปัตยกรรม',
  'java.col.type': 'ประเภท',
  'java.managed': 'จัดการโดยระบบ',
  'java.system': 'ระบบ',
  'java.empty': 'ไม่พบ JDK ดาวน์โหลดได้จากด้านบน',
  'java.legacyNote': 'Minecraft 1.12 และเก่ากว่าต้องใช้ Java 8 บน Apple Silicon ให้ติดตั้ง Java 8 สำหรับ x86_64 (ทำงานผ่าน Rosetta 2)',

  // logs page
  'logs.title': 'บันทึก',
  'logs.noSelection': 'ยังไม่ได้เลือกอินสแตนซ์',
  'logs.noSelectionDesc': 'เปิดอินสแตนซ์เพื่อดูบันทึกของที่นี่',
  'logs.createInstance': '+ สร้างอินสแตนซ์',
  'logs.selectInstance': 'เลือก {name}',
  'logs.autoRefresh': 'รีเฟรชอัตโนมัติ',
  'logs.empty': 'ยังไม่มีบันทึก เปิดเกมเพื่อสร้างบันทึก',

  // servers page
  'servers.title': 'เซิร์ฟเวอร์',
  'servers.addServer': 'เพิ่มเซิร์ฟเวอร์',
  'servers.address': 'ที่อยู่:พอร์ต',
  'servers.name': 'ชื่อ (ไม่บังคับ)',
  'servers.online': 'ออนไลน์',
  'servers.offline': 'ออฟไลน์',
  'servers.players': 'ผู้เล่น {online}/{max}',
  'servers.clickTest': 'คลิกทดสอบเพื่อตรวจสอบ',
  'servers.empty': 'ยังไม่มีเซิร์ฟเวอร์ เพิ่มได้จากด้านบน',

  // settings page
  'settings.title': 'การตั้งค่า',
  'settings.defaultMaxMemory': 'หน่วยความจำสูงสุดเริ่มต้น (MB)',
  'settings.defaultMinMemory': 'หน่วยความจำต่ำสุดเริ่มต้น (MB)',
  'settings.autoSwitchLogs': 'สลับไปแท็บบันทึกอัตโนมัติหลังเปิดเกม',
  'settings.save': 'บันทึกการตั้งค่า',
  'settings.language': 'ภาษา',
  'settings.languageDesc': 'ภาษาของอินเทอร์เฟซ — การเปลี่ยนแปลงมีผลทันที',
  'settings.saved': 'บันทึกการตั้งค่าแล้ว',
};

export default th;
