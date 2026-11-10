# Страна возможностей — порт на Rust + Bevy

Порт игры «Страна возможностей» (e^cha, 2014, GameMaker: Studio 1.x) на Rust и Bevy 0.20.

Логика игры переписана с декомпилированного GML. Ресурсы уже включены в репозиторий;
при необходимости их можно повторно извлечь из оригинала: программа распаковывает CAB-архив
внутри `StranaV.exe`, разбирает `data.win` и раскладывает файлы в папке `assets/`.
Игра читает ресурсы из `assets/` или использует вшитые при сборке.

## Запуск

```sh
cargo run                                   # запуск с включёнными ресурсами
cargo run -- extract путь/к/StranaV.exe     # извлечь вручную (подходит и data.win); третий аргумент — папка
cargo run -- путь/к/assets                  # запустить с другой папкой ресурсов (или STRANA_ASSETS)
```

Что лежит в `assets/`:
- `sprites/*.png`, `backgrounds/*.png`, `fonts/*.png` — изображения, нарезанные из текстурных атласов;
- `game.json` — строки, метрики глифов, маски столкновений, объекты, комнаты и расстановка в них.

`assets/` включена в репозиторий и автоматически вшивается в сборки.

### Один исполняемый файл

Если при сборке рядом с `Cargo.toml` есть `assets/`, `build.rs` вшивает её в бинарник
(другую папку можно указать через `STRANA_EMBED_ASSETS`). Такой файл запускается сам по себе,
без `StranaV.exe` и `assets/`:

```sh
cargo run -- extract StranaV.exe   # один раз
cargo build --release              # target/release/strana — самодостаточный
```

После повторного извлечения просто пересоберите проект. Порядок поиска ресурсов при запуске:
сначала папка из аргумента или `STRANA_ASSETS`, затем вшитые ресурсы, затем `./assets`,
и последним шагом — извлечение из найденного `StranaV.exe`.

### Сборка под Windows и FreeBSD

Кросс-компиляция Windows из Linux через [zig](https://ziglang.org) и `cargo-zigbuild`:

```sh
rustup target add x86_64-pc-windows-gnu
cargo install cargo-zigbuild               # zig должен быть в PATH
cargo zigbuild --release --target x86_64-pc-windows-gnu    # target/x86_64-pc-windows-gnu/release/strana.exe
```

Для FreeBSD используйте `cargo build --release` непосредственно на FreeBSD;
GitHub Actions собирает её в виртуальной машине FreeBSD 14.

Ресурсы вшиваются так же, как в Linux-сборку. Версия для Windows открывается без консольного окна.
Если её запустить из `cmd`, сообщения выводятся в эту консоль, а ошибки показываются в окне.
Нужна Windows 10 или новее. Сборка для FreeBSD рассчитана на FreeBSD 14 и новее, ей нужны X11 или Wayland
и драйвер Vulkan или OpenGL.

### Android

`android/build.sh` собирает `target/android/strana.apk`. Игра там работает как NativeActivity без Java-кода.
Ресурсы обязательно вшиваются, поэтому перед сборкой нужна папка `assets/`.

```sh
rustup target add aarch64-linux-android x86_64-linux-android
android/build.sh                   # arm64-v8a (телефоны) и x86_64 (эмулятор)
ABIS=arm64-v8a android/build.sh    # только для телефонов
adb install -r target/android/strana.apk
```

Скрипту нужны NDK, build-tools и платформа (`android.jar`) в `$ANDROID_HOME` (по умолчанию `~/Android/Sdk`).
APK подписывается отладочным ключом `~/.android/debug.keystore`; если ключа нет, скрипт его создаёт.
На экране касание работает как левая кнопка мыши, а кнопка «Назад» в диалогах — как Esc.
Экран всегда горизонтальный. Сохранение лежит во внутренней папке приложения.

Сохранение лежит в `$XDG_DATA_HOME/StranaV/save.json`, а на Windows в `%LOCALAPPDATA%\StranaV\save.json`.

## Автоматические сборки

GitHub Actions собирает Linux, Windows, FreeBSD и Android при изменениях в `main`,
запросах на слияние и ручном запуске. Файлы доступны в артефактах запуска.
Тег вида `v*` дополнительно публикует их в GitHub Releases. Android APK подписывается
временным отладочным ключом, поэтому такие сборки не предназначены для публикации
в магазине и не обязательно смогут обновить установленную ранее версию.

## Устройство

| файл | что внутри |
|---|---|
| `build.rs` | вшивает локальную `assets/` в исполняемый файл |
| `src/assets.rs` | извлечение: CAB из `StranaV.exe` → `data.win` → `assets/`; загрузка вшитых ресурсов или папки |
| `src/gmdata.rs` | разбор `data.win`: строки, текстурные страницы, спрайты с масками, фоны, шрифты, объекты, комнаты |
| `src/canvas.rs` | программный рендер с семантикой `draw_*` из GameMaker: альфа, `bm_add`, растровые шрифты, `#` как перевод строки |
| `src/game.rs` | цикл раннера: alarm, события мыши, step, движение, коллизии по маскам, отрисовка по depth, комнаты, `show_message`/`show_question` |
| `src/objects.rs` | события объектов (`o_button`, `listovki`, `downloader`, `programgame`, `pcc`, `uprav`, …) |
| `src/scripts.rs` | скрипты (`BAction`, `GetWork`, `GetHome`, `_hour`, `_day`, `doptext`, …) и HUD объекта `gas` |
| `src/particles.rs` | система частиц финальной сцены |
| `src/txt.rs` | номера строк в таблице строк игры (`strings` в `game.json`) |
| `src/lib.rs` | Bevy: окно 800×600 с масштабированием, шаг логики 30 Гц (`FixedUpdate`, как room_speed), ввод с мыши и касаний, вывод кадра; точка входа для Android |
| `src/main.rs` | запуск на десктопе |
| `android/` | манифест и скрипт сборки APK |

`STRANA_DUMP_ROOMS=папка cargo run` без окна отрисовывает все комнаты в PNG. Это нужно для сверки с оригиналом.

## Отличия от оригинала

- `show_message` в оригинале открывал системное окно Windows. Здесь вместо него диалог внутри игры (Enter/Esc).
- Встроенные текстуры частиц GameMaker (line, flare) нарисованы заново процедурно.
- Поведение оригинала сохранено вместе с его ошибками, кроме одной. В оригинале часы шли уже на титульном экране, и через 24 секунды автосохранение перезаписывало прежний сейв новой игрой. Здесь часы начинают идти только с началом игры.
