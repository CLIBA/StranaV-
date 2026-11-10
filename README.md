# Страна возможностей — порт на Rust + Bevy

**English:** A Rust and Bevy port of the 2014 GameMaker game *Strana Vozmozhnostey*. Requires assets extracted from your own copy of the original game.

**Русский:** Порт игры «Страна возможностей» 2014 года с GameMaker на Rust и Bevy. Для запуска нужны ресурсы, извлечённые из вашей копии оригинальной игры.

Порт игры «Страна возможностей» (e^cha, 2014, GameMaker: Studio 1.x) на Rust и Bevy 0.20.

Логика игры переписана с декомпилированного GML. Формат GameMaker порту нужен
только один раз, при извлечении: он сам распаковывает CAB-архив внутри `StranaV.exe`,
разбирает `data.win` и раскладывает ресурсы по обычным файлам в папке `assets/`.
Дальше игра читает только `assets/`.

## Запуск

```sh
cargo run                                   # при первом запуске сам найдёт StranaV.exe и извлечёт assets/
cargo run -- extract путь/к/StranaV.exe     # извлечь вручную (подходит и data.win); третий аргумент — папка
cargo run -- путь/к/assets                  # запустить с другой папкой ресурсов (или STRANA_ASSETS)
```

Что лежит в `assets/`:
- `sprites/*.png`, `backgrounds/*.png`, `fonts/*.png` — изображения, нарезанные из текстурных атласов;
- `game.json` — строки, метрики глифов, маски столкновений, объекты, комнаты и расстановка в них.

`assets/` извлекается из вашей копии игры и в репозиторий не коммитится (см. `.gitignore`).

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

Кросс-компиляция из Linux через [zig](https://ziglang.org) и `cargo-zigbuild`:

```sh
rustup target add x86_64-pc-windows-gnu x86_64-unknown-freebsd
cargo install cargo-zigbuild               # zig должен быть в PATH
cargo zigbuild --release --target x86_64-pc-windows-gnu    # target/x86_64-pc-windows-gnu/release/strana.exe
cargo zigbuild --release --target x86_64-unknown-freebsd   # target/x86_64-unknown-freebsd/release/strana
```

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
