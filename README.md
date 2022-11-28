# apple_releases

A simple CLI app that scrapes the [Software Releases] section of the Apple Developer News and Updates site,
parsing out the details for display in a terminal.

## Requirements

- Requires rust nightly

## Example

```text
$ cargo run +nightly
2022-11-15 - iOS 16.2 beta 3 (20C5049e) - https://developer.apple.com/go/?id=ios-16.2-rn
2022-11-15 - iPadOS 16.2 beta 3 (20C5049e) - https://developer.apple.com/go/?id=iPadOS-16.2-rn
2022-11-15 - macOS 13.1 beta 3 (22C5050e) - https://developer.apple.com/go/?id=macos-13.1-rn
2022-11-15 - watchOS 9.2 beta 3 (20S5348d) - https://developer.apple.com/go/?id=watchos-9.2-rn
2022-11-15 - tvOS 16.2 beta 3 (20K5348d) - https://developer.apple.com/go/?id=tvos-16.2-rn
2022-11-01 - Xcode 14.1 (14B47b) - https://developer.apple.com/go/?id=xcode-14.1-sdk-rn
2022-10-24 - watchOS 9.1 (20S75) - https://developer.apple.com/go/?id=watchos-9.1-rn
2022-10-24 - tvOS 16.1 (20K71) - https://developer.apple.com/go/?id=tvos-16.1-rn
2022-09-26 - Xcode 14.0.1 (14A400) - https://developer.apple.com/go/?id=xcode-14.0.1-sdk-rn
2022-09-12 - tvOS 16 (20J373) - https://developer.apple.com/go/?id=tvos-16-rn
2022-07-20 - tvOS 15.6 (19M65) - https://developer.apple.com/go/?id=tvos-15.6-sdk-rn
```

## 📄 License

This repo is licensed under the MIT License. See the [LICENSE](LICENSE.md) file for rights and limitations.

[Software Releases]: https://developer.apple.com/news/releases/
