// https://leetcode.com/problems/course-schedule-ii/solutions/516362/c-how-to-go-from-course-scheduler-1-course-scheduler-2/
public class Solution
{
    public bool CanFinish(int numCourses, int[][] prerequisites) 
    {
        // Init graph data structure and integer array used 
        // to count the number of outstanding pre-reqs for a given class
        List<int>[] preToPost = new List<int>[numCourses];
        int[] preReqCount = new int[numCourses];

        // populate the graph and array mentioned above
        for (int i = 0; i < numCourses; i++) preToPost[i] = new List<int>();
        foreach (int[] p in prerequisites)
        {
            int prereq = p[1];
            int nextCourse = p[0];
            preToPost[prereq].Add(nextCourse);
            preReqCount[nextCourse]++;
        }
        
        return !CycleExists(preToPost, preReqCount);
    }
    
    private bool CycleExists(List<int>[] graph, int[] preReqCount)
    {
        // Checks if a cycle exists. 
        Queue<int> q = new Queue<int>();

        // The queue will be initially populated with 
        // courses that have no prereqs                
        for (int i = 0; i < preReqCount.Length; i++)
            if (preReqCount[i] == 0)
                q.Enqueue(i);

        while (q.Count > 0)
        {
            int size = q.Count;

            for (int i = 0; i < size; i++)
            {
                int course = q.Dequeue();

                // explore each of the courses that have
                // the current course as a prereq
                foreach (int next in graph[course])
                {
                    // Now that we have taken course "course", 
                    // we can remove it as a required preReq for the future courses.

                    // Once a future course has 0 outstanding prereqs, we can add it to 
                    // our queue for processing
                    preReqCount[next]--;
                    if (preReqCount[next] == 0)
                        q.Enqueue(next);
                }
            }
        }

        // If all courses have no satisfied all of their prereqs, 
        // then there must be a cycle. Return true IFF all courses have no more outstanding prereqs
        return !preReqCount.All(x => x <= 0);
    }
        
}