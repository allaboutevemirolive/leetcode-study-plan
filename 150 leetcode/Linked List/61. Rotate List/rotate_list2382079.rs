// https://leetcode.com/problems/rotate-list/solutions/2382079/c-solution/
void total(struct ListNode** ans, struct ListNode* first, struct ListNode* head, int* all, int* backadd, int k){
    if (head == NULL){
        return;
    }
    *all += 1;
    total(ans, first, head->next, all, backadd, k);
    
    *backadd += 1;
    
    if (k % *all == 0){
        return;
    }
    if (*backadd == 1){
        head->next = first;
    }
    if (*backadd == k % *all){
        *ans = head;
        return;
    }
    if (*backadd == (k % *all)+1){
        head->next = NULL;
    }
    return;
}

struct ListNode* rotateRight(struct ListNode* head, int k){
    if(head == NULL){
        return NULL;
    }
    if(k == 0){
        return head;
    }
    struct ListNode** ans = malloc(sizeof(struct ListNode*));
    int all = 0;
    int backadd = 0;
    total(ans, head, head, &all, &backadd, k);
    struct ListNode* rust = *ans;
    free(ans);
    if (k % all == 0){
        return head;
    }
    return rust;
}