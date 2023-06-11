// https://leetcode.com/problems/implement-trie-prefix-tree/solutions/3444021/c-python-solution/
const int LOWERCASE_ALPHABET = 26;
const char letter_a = 'a';

struct TrieNode {
    TrieNode* children[LOWERCASE_ALPHABET] = {nullptr};
    bool is_word = false;
};


class Trie {

private:
    TrieNode* root;
public:
    Trie() {
        root = new TrieNode();
    }
    
    void insert(string word) {
        
        TrieNode* current = root;

        for(int i=0; i<word.length(); i++) {
            int key = word[i]-letter_a;
            if(current->children[key] == nullptr){
                current->children[key] = new TrieNode();
            }
            current = current->children[key];
        } 

        current->is_word = true;
        
    }
    
    bool search(string word) {
        
        TrieNode* current = root;

        for(int i=0; i<word.length(); i++) {
            int key = word[i]-letter_a;

            if(current->children[key] == nullptr) {
                return false;
            }
            current = current->children[key];
        }

        return current->is_word;
    }
    
    bool startsWith(string prefix) {

        TrieNode* current = root;

        for(int i=0; i<prefix.length(); i++) {
            int key = prefix[i]-letter_a;

            if(current->children[key] == nullptr) {
                return false;
            }
            current = current->children[key];
        }

        return true;
        
    }
};