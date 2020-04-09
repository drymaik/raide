function probe(zahl1, zahl2, text, tabelle)
    zahl3 = zahl1 + zahl2
    zahl4 = zahl1 - zahl2

    print(text)

    if tabelle ~= nil then
        print(tabelle.eintrag)
    end

    return zahl3,zahl4
end

probe(10, 20, "Hallo ", {eintrag = "Welt"}) -- erlaubter Funktionsaufruf

x,y = probe(10,20) -- ebenfalls erlaubter Aufruf, text und tabelle sind nil.
