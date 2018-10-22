-- This script calculates transcript length of each gene from input GTF file.
-- The result is written to standard output. 
-- Usage: runghc get_gene_length.hs <Input GTF file>

import System.Environment
import Control.Monad
import Control.Applicative
import Data.List.Split
import Data.List(foldl')
import Text.Regex.Posix
import qualified Data.Map as M

main = do
    gtf <- head <$> getArgs
    contents <- readFile gtf
    let result = M.assocs $ foldl' (\acc line -> case extractValue line of
                                                    Just val -> updateLength val acc
                                                    Nothing -> acc) M.empty $ lines contents
    forM_ result $ putStrLn . (\tuple -> fst tuple ++ "\t" ++ (show $ snd tuple))

extractValue::String -> Maybe (String, Int)
extractValue line = let elems = splitOn "\t" line
                        region = elems !! 2                          
                        start = (read::String -> Int) $ elems !! 3
                        end = (read::String -> Int) $ elems !! 4
                        name = (elems !! 8  =~ "transcript_id \"N._[0-9]+.[0-9]*\"" ::String) =~ "N._[0-9]+.[0-9]*" ::String
                    in if region == "exon" && name /= ""
                        then Just (name, end - start)
                        else Nothing

updateLength::(String, Int) -> M.Map String Int -> M.Map String Int
updateLength tuple map = let key = fst tuple
                             val = snd tuple
                         in if M.member key map
                                then M.adjust ((+) val) key map
                                else M.insert key val map 
