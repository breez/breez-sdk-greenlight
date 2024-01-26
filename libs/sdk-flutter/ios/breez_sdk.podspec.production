# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint breez_sdk.podspec` to validate before publishing.
Pod::Spec.new do |s|
  s.name             = 'breez_sdk'
  s.version          = '0.2.15'
  s.summary          = 'BreezSDK flutter plugin.'
  s.description      = <<-DESC
  BreezSDK flutter plugin.
                       DESC
  s.homepage         = 'https://breez.technology'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Breez' => 'contact@breez.technology' }
  s.source           = { :git => "https://github.com/breez/breez-sdk-flutter.git", :tag => "#{s.version}" }
  s.source_files = 'Classes/**/*'
  s.dependency 'Flutter'
  s.platform = :ios, '11.0'
  s.static_framework = true

  # Flutter.framework does not contain a i386 slice.
  s.pod_target_xcconfig = {'STRIP_STYLE' => 'non-global', 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386' }
  s.swift_version = '5.0'
  s.dependency "BreezSDK", "#{s.version}"
end
