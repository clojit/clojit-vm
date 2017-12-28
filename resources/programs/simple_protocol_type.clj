(do
    (defprotocol REST
        (GET [self]))
    (deftype API [a]
        REST
        (GET [self] a))
    (GET (->API 1001)))