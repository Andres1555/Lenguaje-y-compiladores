import Control.Concurrent (threadDelay)

encenderLuz :: Int -> String -> IO ()
encenderLuz id color = putStrLn ("Semaforo " ++ show id ++ " -> " ++ color)

prioridadEmergencia :: Int -> IO Bool
prioridadEmergencia _ = return True  -- Simulación

main :: IO ()
main = loop
  where
    loop = do
        emergencia <- prioridadEmergencia(2)
        if emergencia
            then do
                encenderLuz 2 "verde"
                threadDelay (40 * 1000000)
                encenderLuz 2 "amarillo"
                threadDelay (5 * 1000000)
                encenderLuz 2 "rojo"
                threadDelay (30 * 1000000)
            else do
                encenderLuz 2 "verde"
                threadDelay (25 * 1000000)
                encenderLuz 2 "amarillo"
                threadDelay (5 * 1000000)
                encenderLuz 2 "rojo"
                threadDelay (25 * 1000000)
        loop
