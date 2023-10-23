import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  const MethodChannel channel = MethodChannel('breez_sdk');

  TestWidgetsFlutterBinding.ensureInitialized();
  final binding = TestDefaultBinaryMessengerBinding.instance;

  setUp(() {
    binding.defaultBinaryMessenger.setMockMethodCallHandler(channel, (message) async => '42');
  });

  tearDown(() {
    binding.defaultBinaryMessenger.setMockMethodCallHandler(channel, (message) => null);
  });

  test('getPlatformVersion', () async {});
}
