(do
  (defprotocol REST
    (GET [self]))

  (deftype API [a]
    REST
    (GET [self] a))

  (let [a "jdfkldafjölkdafjöakjfdaölkdfjaölfjda"
        b :random-key-test-key
        c 34398283492
        d 1921.3432]
      (GET (->API 5))))


