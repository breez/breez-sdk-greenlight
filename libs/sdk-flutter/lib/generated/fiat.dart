// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.32.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

/// Details about a supported currency in the fiat rate feed
class CurrencyInfo {
  final String name;
  final int fractionSize;
  final int? spacing;
  final Symbol? symbol;
  final Symbol? uniqSymbol;
  final List<LocalizedName>? localizedName;
  final List<LocaleOverrides>? localeOverrides;

  const CurrencyInfo({
    required this.name,
    required this.fractionSize,
    this.spacing,
    this.symbol,
    this.uniqSymbol,
    this.localizedName,
    this.localeOverrides,
  });

  @override
  int get hashCode =>
      name.hashCode ^
      fractionSize.hashCode ^
      spacing.hashCode ^
      symbol.hashCode ^
      uniqSymbol.hashCode ^
      localizedName.hashCode ^
      localeOverrides.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CurrencyInfo &&
          runtimeType == other.runtimeType &&
          name == other.name &&
          fractionSize == other.fractionSize &&
          spacing == other.spacing &&
          symbol == other.symbol &&
          uniqSymbol == other.uniqSymbol &&
          localizedName == other.localizedName &&
          localeOverrides == other.localeOverrides;
}

/// Wrapper around the [CurrencyInfo] of a fiat currency
class FiatCurrency {
  final String id;
  final CurrencyInfo info;

  const FiatCurrency({
    required this.id,
    required this.info,
  });

  @override
  int get hashCode => id.hashCode ^ info.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FiatCurrency && runtimeType == other.runtimeType && id == other.id && info == other.info;
}

/// Locale-specific settings for the representation of a currency
class LocaleOverrides {
  final String locale;
  final int? spacing;
  final Symbol symbol;

  const LocaleOverrides({
    required this.locale,
    this.spacing,
    required this.symbol,
  });

  @override
  int get hashCode => locale.hashCode ^ spacing.hashCode ^ symbol.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LocaleOverrides &&
          runtimeType == other.runtimeType &&
          locale == other.locale &&
          spacing == other.spacing &&
          symbol == other.symbol;
}

/// Localized name of a currency
class LocalizedName {
  final String locale;
  final String name;

  const LocalizedName({
    required this.locale,
    required this.name,
  });

  @override
  int get hashCode => locale.hashCode ^ name.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LocalizedName &&
          runtimeType == other.runtimeType &&
          locale == other.locale &&
          name == other.name;
}

/// Denominator in an exchange rate
class Rate {
  final String coin;
  final double value;

  const Rate({
    required this.coin,
    required this.value,
  });

  @override
  int get hashCode => coin.hashCode ^ value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Rate && runtimeType == other.runtimeType && coin == other.coin && value == other.value;
}

/// Settings for the symbol representation of a currency
class Symbol {
  final String? grapheme;
  final String? template;
  final bool? rtl;
  final int? position;

  const Symbol({
    this.grapheme,
    this.template,
    this.rtl,
    this.position,
  });

  @override
  int get hashCode => grapheme.hashCode ^ template.hashCode ^ rtl.hashCode ^ position.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Symbol &&
          runtimeType == other.runtimeType &&
          grapheme == other.grapheme &&
          template == other.template &&
          rtl == other.rtl &&
          position == other.position;
}