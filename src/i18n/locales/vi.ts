/** Tiếng Việt */
const vi = {
  'app.name': 'CraftCraft',

  // navigation
  'nav.instances': 'Bản cài',
  'nav.newInstance': 'Bản cài mới',
  'nav.java': 'Java',
  'nav.logs': 'Nhật ký',
  'nav.servers': 'Máy chủ',
  'nav.settings': 'Cài đặt',

  // generic actions
  'common.cancel': 'Hủy',
  'common.create': 'Tạo',
  'common.save': 'Lưu',
  'common.delete': 'Xóa',
  'common.del': 'Xóa',
  'common.edit': 'Sửa',
  'common.launch': 'Khởi chạy',
  'common.all': 'Tất cả',
  'common.filter': 'Lọc',
  'common.add': 'Thêm',
  'common.test': 'Kiểm tra',
  'common.refresh': 'Làm mới',
  'common.downloading': 'Đang tải xuống…',
  'common.optional': 'không bắt buộc',

  // instances
  'instances.title': 'Bản cài',
  'instances.count_one': '{n} bản cài',
  'instances.count_other': '{n} bản cài',
  'instances.filteredBy': 'đã lọc: {tag}',
  'instances.empty.title': 'Chưa có bản cài nào',
  'instances.empty.desc': 'Tạo bản cài Minecraft đầu tiên của bạn để bắt đầu.',
  'instances.deleteConfirm': 'Xóa bản cài này? Dữ liệu game sẽ được giữ lại.',
  'instances.launchFailed': 'Khởi chạy thất bại: {msg}',
  'instances.addTag': 'Thêm thẻ…',
  'instances.newCard': 'Bản cài mới',

  // create-instance wizard
  'wizard.title': 'Tạo bản cài',
  'wizard.name': 'Tên',
  'wizard.namePlaceholder': 'Thế giới sinh tồn của tôi',
  'wizard.version': 'Phiên bản',
  'wizard.versionPlaceholder': '1.21',
  'wizard.notInManifest': 'Không có trong danh mục — dùng nguyên trạng',
  'wizard.loader': 'Trình tải',
  'wizard.accountName': 'Tên tài khoản',
  'wizard.maxMemory': 'Bộ nhớ tối đa (MB)',
  'wizard.javaRuntime': 'Môi trường Java',
  'wizard.jdkAvailable': 'Có {n} JDK khả dụng',
  'wizard.noJdk': 'Không tìm thấy JDK — tải xuống bên dưới',
  'wizard.createFailed': 'Tạo thất bại: {msg}',
  'wizard.create': 'Tạo bản cài',

  // java runtime page
  'java.title': 'Môi trường Java',
  'java.downloadTemurin': 'Tải Temurin',
  'java.installed': 'JDK đã cài',
  'java.col.version': 'Phiên bản',
  'java.col.path': 'Đường dẫn',
  'java.col.vendor': 'Nhà cung cấp',
  'java.col.arch': 'Kiến trúc',
  'java.col.type': 'Loại',
  'java.managed': 'Được quản lý',
  'java.system': 'Hệ thống',
  'java.empty': 'Không tìm thấy JDK. Tải xuống ở trên.',
  'java.legacyNote': 'Minecraft 1.12 trở về trước cần Java 8. Trên Apple Silicon hãy cài bản Java 8 x86_64 (chạy qua Rosetta 2).',

  // logs page
  'logs.title': 'Nhật ký',
  'logs.noSelection': 'Chưa chọn bản cài',
  'logs.noSelectionDesc': 'Khởi chạy một bản cài để xem nhật ký tại đây.',
  'logs.createInstance': '+ Tạo bản cài',
  'logs.selectInstance': 'Chọn {name}',
  'logs.autoRefresh': 'Tự động làm mới',
  'logs.empty': 'Chưa có nhật ký. Khởi chạy game để tạo nhật ký.',

  // servers page
  'servers.title': 'Máy chủ',
  'servers.addServer': 'Thêm máy chủ',
  'servers.address': 'địa chỉ:cổng',
  'servers.name': 'tên (không bắt buộc)',
  'servers.online': 'Trực tuyến',
  'servers.offline': 'Ngoại tuyến',
  'servers.players': '{online}/{max} người chơi',
  'servers.clickTest': 'Nhấn Kiểm tra để xem',
  'servers.empty': 'Không có máy chủ. Thêm ở trên.',

  // settings page
  'settings.title': 'Cài đặt',
  'settings.defaultMaxMemory': 'Bộ nhớ tối đa mặc định (MB)',
  'settings.defaultMinMemory': 'Bộ nhớ tối thiểu mặc định (MB)',
  'settings.autoSwitchLogs': 'Tự chuyển sang tab Nhật ký sau khi khởi chạy',
  'settings.save': 'Lưu cài đặt',
  'settings.language': 'Ngôn ngữ',
  'settings.languageDesc': 'Ngôn ngữ giao diện — thay đổi áp dụng ngay',
  'settings.saved': 'Đã lưu cài đặt',
};

export default vi;
