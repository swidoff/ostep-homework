1. Compute the response time and turnaround time when running three jobs of length 200 with the SJF and FIFO
 schedulers. 
 
    **FIFO**
 
    * Job   0 -- Response: 0.00    Turnaround 200.00
    * Job   1 -- Response: 200.00  Turnaround 400.00
    * Job   2 -- Response: 400.00  Turnaround 600.00
    * Average -- Response: 200.00  Turnaround 400.00
    
    **SJF**
     
    * Job   0 -- Response: 0.00    Turnaround 200.00
    * Job   1 -- Response: 200.00  Turnaround 400.00
    * Job   2 -- Response: 400.00  Turnaround 600.00
    * Average -- Response: 200.00  Turnaround 400.00
    
2. Now do the same but with jobs of different lengths: 100, 200, and 300.

    **FIFO**
 
    * Job   0 -- Response: 0.00    Turnaround 100.00
    * Job   1 -- Response: 100.00  Turnaround 300.00
    * Job   2 -- Response: 300.00  Turnaround 600.00
    * Average -- Response: 133.33  Turnaround 333.33
    
    **SJF**
     
    * Job   0 -- Response: 0.00    Turnaround 100.00
    * Job   1 -- Response: 100.00  Turnaround 300.00
    * Job   2 -- Response: 300.00  Turnaround 600.00
    * Average -- Response: 133.33  Turnaround 333.33

3. Now do the same, but also with the RR scheduler and a time-slice of 1.

    * Job   0 -- Response: 0.00    Turnaround 300.00
    * Job   1 -- Response: 1.00    Turnaround 500.00
    * Job   2 -- Response: 2.00    Turnaround 600.00
    * Average -- Response: 1.00    Turnaround 466.67

4. For what types of workloads does SJF deliver the same turnaround times as FIFO?

    When the jobs arrive already in order from shortest to longest.

5. For what types of workloads and quantum lengths does SJF deliver the same response times as RR?

    quantum = min(job length), jobs already arrive SJF.

6. What happens to response time with SJF as job lengths increase? Can you use the simulator to demonstrate the trend?

    Response times increase.

7. What happens to response time with RR as quantum lengths increase? 
 Can you write an equation that gives the worst-case response time, given N jobs?

    average response times increase

    quantum * N (assuming all jobs have len > quantum)