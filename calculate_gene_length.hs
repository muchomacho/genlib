import Control.Monad
import Data.List.Split
import Data.List(foldl')
import Text.Regex.Posix
import qualified Data.Map as M

main = do
    contents <- readFile "refseq_genes_hg19.gtf"
    let result = M.assocs $ foldl' (\acc line -> case extract_values line of
                                                    Just val -> update_length val acc
                                                    Nothing -> acc) M.empty $ lines contents
    forM_ result $ putStrLn . (\tuple -> fst tuple ++ "\t" ++ (show $ snd tuple))

extract_values:: String -> Maybe (String, Int)
extract_values line = let split_words = splitOn "\t" line
                          start = (read::String -> Int) $ split_words !! 3
                          end = (read::String -> Int) $ split_words !! 4
                          region = split_words !! 2
                          name = (split_words !! 8  =~ "transcript_id \"(N._[0-9]+.[0-9])\"" ::String) =~ "N._[0-9]+.[0-9]" ::String
                      in if region == "exon" 
                            then Just (name, end - start)
                            else Nothing

update_length:: (String, Int) -> M.Map String Int -> M.Map String Int
update_length tuple map = let key = fst tuple
                              val = snd tuple
                          in if M.member key map
                                then M.adjust ((+) val) key map
                                else M.insert key val map 
