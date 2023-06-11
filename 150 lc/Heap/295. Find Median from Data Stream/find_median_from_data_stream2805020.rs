// https://leetcode.com/problems/find-median-from-data-stream/solutions/2805020/daily-leetcoding-challenge-november-day-12/
// Header files for ordered multiset
# include   <ext/pb_ds/assoc_container.hpp>
# include  <ext/pb_ds/tree_policy.hpp>
using namespace __gnu_pbds;
typedef tree<int, null_type, less_equal<int>, rb_tree_tag, tree_order_statistics_node_update>  ordered_multiset;\
// find_by_order(k) : k-th element in the multiset
class MedianFinder {
public:
    ordered_multiset s;
   MedianFinder() {
        
    }
    
    void addNum(int num) {
        s.insert(num);
    }
    
    double findMedian() {
        int x=s.size();
        double ans=0;
        if(x%2==0) // for even size of array
            ans=(*s.find_by_order(x/2 -1)+*s.find_by_order(x/2))/2.00;
        else      // for odd size
            ans=*s.find_by_order(x/2);
        return ans;
        
    }
};