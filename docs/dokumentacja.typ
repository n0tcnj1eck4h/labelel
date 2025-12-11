#set page(margin: 1in)
#set par(justify: true)
#set text(lang: "pl")
#set heading(numbering: "1.1")


#align(center + horizon)[
  // #v(6cm)
  #text(size: 24pt, weight: "bold")[Dokumentacja programu Labelel]
]

#place(
  bottom + left,
  text(size: 10pt)[2025/2026r.]
)

#pagebreak()
#outline()
#pagebreak()

= Opis
Program Labelel pozwala na przeglądanie i modyfikowanie etykiet istniejących zestawów danych w formacie YOLOv11,
skupiając się na ergonomii i wydajności etykietowania.
Program nie wymaga instalacji i nie korzysta z technologii webowych,
dzięki czemu może być używany na sprzęcie z niskimi zasobami.

Skróty klawiszowe są przystosowane w sposób przypominający sterowanie w grach komputerowych --
lewa ręka nawiguje między obrazami i narzędziami klawiszami `WASD`, natomiast prawa ręka pozostaje na myszce.

Program posiada trzy narzędzia:
  - Stamp tool -- bounding box jest ,,przyklejony'' do kursora myszy, a jego rozmiar i proporcje można zmienić kółkiem myszy.
              Zaznaczony obszar jest zatwierdzany spacją. W połączeniu z opcją ,,Quick advance'' idealnie nadaje się do oznaczania
              serii klatek wyciągniętych z trzęsącego się nagrania. 
  - Drag tool -- standardowe przeciąganie bounding box'a spotykane w każdym programie do etykietowania obrazów.
  - Edit tool -- zmienianie rozmiaru i pozycji istniejących etykiet

Za kopiowanie i usuwanie istniejących etykiet odpowiadają skróty klawiszowe odpowiednio `C` i `X`.
#figure(
  image("a.png", width: 100%),
  caption: [Interfejs programu],
)

#pagebreak()
= Instrukcja obsługi
Aby uruchomić program wystarczy dwukrotnie wcisnąć plik wykonywalny.
Alternatywnie jeżeli chcemy od razu otworzyć wybrany zestaw danych możemy podać do niego ścieżkę: `labelel.exe ./zestaw/data.yaml`.

== Tworzenie projektu
Ze względu na relatywnie sztywną strukturę katalogów formatu YOLOv11 program może otwierać tylko istniejące projektu.
Szkielet projektu można utworzyć ręcznie wklejając poniży tekst to pliku `data.yaml` i wklejeniu interesujących nas
obrazów do katalogu `images`:

```yaml
test: ./images
train: ./images
val: ./images
names:
    0: klasa1
    1: klasa2
    2: klasa3
```

== Otwieranie i zapisywanie projektu
Projekt można otworzyć wciskając przycisk z ikonką folderu w prawym dolnym rogu ekranu i wybierająć plik `data.yaml` naszego zestawu.
#figure(
  image("load.png", width: 50%),
  caption: [Przycisk otwierania projektu],
)

Przycisk otwiera natywny dialog wyboru pliku.

#figure(
  image("choice.png", width: 70%),
  caption: [Okno dialogowe wyboru pliku KDE Plasma],
)

Zapisywanie projektu odbywa się za pomocą przycisku dyskietki obok przycisku otwierania projektu.
Opcja ,,Zapisz jako'' obecnie nie istnieje. 
// #figure(
//   image("save.png", width: 50%),
//   caption: [Przycisk zapisywania projektu],
// )

#pagebreak()
== Dodawanie klas
W prawym rogu ekranu widnieje lista klas (Labels), gdzie można wybrać aktywną klasę lub dodać nową przyciskiem z ikonką plusa.
#figure(
  image("labels.png", width: 70%),
  caption: [Panel wyboru klas],
)

== Nawigacja
W programie istnieją trzy sposoby nawigacji między obrazami.
Głównym sposobem są klawisze `A` i `D` na klawiaturze, które odpowiednio idą wstecz lub do następnego obrazu.
Obraz można też wybrać z dolnego panelu, klikając na interesujący nas obraz.
#figure(
  image("timeline.png", width: 70%),
  caption: [Wybór obrazu z dolnego panelu],
)

Można też użyć przycisków nawigacyjnych na prawym panelu:
#figure(
  image("nav.png", width: 70%),
  caption: [Przyciski nawigacyjne],
)

== Etykietowanie
Do pracy z etykietami są dostępne trzy narzędzia: Stamp, drag i edit tool.
Aktywne narzędzie można wybrać z prawego panelu albo skrótami klawiszowymi `Q`, `W` i `E`.
#figure(
  image("tools.png", width: 70%),
  caption: [Narzędzia],
)
