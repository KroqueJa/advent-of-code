module Lib (readFileStrict) where

import Control.Exception (catch, IOException)
import qualified Data.ByteString as BS

-- Reads a file as a strict ByteString and returns it
readFileStrictBs :: FilePath -> IO BS.ByteString
readFileStrictBs filePath = catch (BS.readFile filePath) handleError
  where
    handleError :: IOException -> IO BS.ByteString
    handleError e = do
        putStrLn $ "Error reading file: " ++ show e
        return BS.empty

-- Reads a file as a strict String and returns it
readFileStrict :: FilePath -> IO String
readFileStrict filePath = catch (readFile filePath) handleError
  where
    handleError :: IOException -> IO String
    handleError e = do
        putStrLn $ "Error reading file: " ++ show e
        return ""

