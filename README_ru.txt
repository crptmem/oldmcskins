oldmcskins
==========
Серверное приложение для отображения скинов в старых версиях Minecraf (до
1.7.10)

Установка (на сервере)
======================
  $ git clone https://github.com/crptmem/oldmcskins.git && cd oldmcskins
  $ cargo build

Использование
=====
Сервер должен слушать HTTP запросы на порту 80.
На клиенте, редактируйте /etc/hosts на Linux и c:\windows\system32\drivers\etc\hosts
на Windows. В файл добавьте такую запись:
```
IP_СЕРВЕРА_С_OLDMCSKINS skins.minecraft.net
```
Или же, отредактируйте строки в client.jar которые обращаются к
skins.minecraft.net - замените их на IP/домен вашего сервера.
