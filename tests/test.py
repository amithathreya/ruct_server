import requests
import threading
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
import statistics

def make_request(request_id):
    """Make a single request and return the response time"""
    url = "http://127.0.0.1:7878"
    start_time = time.time()
    try:
        response = requests.get(url)
        elapsed_time = time.time() - start_time
        return {
            'request_id': request_id,
            'status_code': response.status_code,
            'time': elapsed_time,
            'success': True
        }
    except requests.RequestException as e:
        elapsed_time = time.time() - start_time
        return {
            'request_id': request_id,
            'error': str(e),
            'time': elapsed_time,
            'success': False
        }

def run_stress_test(num_requests=1000000, concurrent_requests=50000):
    """Run the stress test"""
    print(f"Starting stress test on http://127.0.0.1:7878")
    print(f"Total requests: {num_requests}")
    print(f"Concurrent requests: {concurrent_requests}")
    
    start_time = time.time()
    results = []
    
    with ThreadPoolExecutor(max_workers=concurrent_requests) as executor:
        futures = [executor.submit(make_request, i) for i in range(num_requests)]
        for future in as_completed(futures):
            results.append(future.result())
    
    total_time = time.time() - start_time
    
    # Analyze results
    successful = [r for r in results if r['success']]
    failed = [r for r in results if not r['success']]
    
    print("\nResults:")
    print(f"Total requests: {len(results)}")
    print(f"Successful: {len(successful)}")
    print(f"Failed: {len(failed)}")
    
    if successful:
        times = [r['time'] for r in successful]
        print(f"\nResponse times (seconds):")
        print(f"Min: {min(times):.3f}")
        print(f"Max: {max(times):.3f}")
        print(f"Average: {statistics.mean(times):.3f}")
        print(f"Median: {statistics.median(times):.3f}")
    
    if failed:
        print("\nErrors:")
        for error in failed:
            print(f"Request {error['request_id']}: {error['error']}")
    
    print(f"\nTotal test time: {total_time:.2f} seconds")

if __name__ == "__main__":
    run_stress_test()
