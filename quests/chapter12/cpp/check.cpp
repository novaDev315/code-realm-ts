// Test file for Chapter 12: Hall of Echoes - Message Queues

#include <iostream>
#include <string>
#include <vector>
#include "queue.cpp"

using namespace std;

int main() {
    bool passed = true;

    // Test enqueue and size
    cout << "Testing enqueue and size..." << endl;
    MessageQueue queue1;

    if (queue1.size() != 0) {
        cerr << "size() on empty queue: expected 0, got " << queue1.size() << endl;
        passed = false;
    } else {
        cout << "  size() on empty queue = 0" << endl;
    }

    queue1.enqueue("message1");
    if (queue1.size() != 1) {
        cerr << "size() after 1 enqueue: expected 1, got " << queue1.size() << endl;
        passed = false;
    } else {
        cout << "  size() after 1 enqueue = 1" << endl;
    }

    queue1.enqueue("message2");
    queue1.enqueue("message3");
    if (queue1.size() != 3) {
        cerr << "size() after 3 enqueues: expected 3, got " << queue1.size() << endl;
        passed = false;
    } else {
        cout << "  size() after 3 enqueues = 3" << endl;
    }

    // Test isEmpty
    cout << "\nTesting isEmpty..." << endl;
    MessageQueue queue2;

    if (!queue2.isEmpty()) {
        cerr << "isEmpty() on empty queue: expected true, got false" << endl;
        passed = false;
    } else {
        cout << "  isEmpty() on empty queue = true" << endl;
    }

    queue2.enqueue("test");
    if (queue2.isEmpty()) {
        cerr << "isEmpty() on non-empty queue: expected false, got true" << endl;
        passed = false;
    } else {
        cout << "  isEmpty() on non-empty queue = false" << endl;
    }

    // Test peek
    cout << "\nTesting peek..." << endl;
    MessageQueue queue3;

    if (queue3.peek() != "") {
        cerr << "peek() on empty queue: expected empty string, got \"" << queue3.peek() << "\"" << endl;
        passed = false;
    } else {
        cout << "  peek() on empty queue = \"\" (empty string)" << endl;
    }

    queue3.enqueue("first");
    queue3.enqueue("second");
    string peeked = queue3.peek();
    if (peeked != "first") {
        cerr << "peek() should return first message: expected \"first\", got \"" << peeked << "\"" << endl;
        passed = false;
    } else {
        cout << "  peek() returns first message = \"first\"" << endl;
    }

    if (queue3.size() != 2) {
        cerr << "peek() should not remove: expected size 2, got " << queue3.size() << endl;
        passed = false;
    } else {
        cout << "  peek() does not remove (size still 2)" << endl;
    }

    // Test dequeue FIFO order
    cout << "\nTesting dequeue (FIFO order)..." << endl;
    MessageQueue queue4;

    if (queue4.dequeue() != "") {
        cerr << "dequeue() on empty queue: expected empty string" << endl;
        passed = false;
    } else {
        cout << "  dequeue() on empty queue = \"\" (empty string)" << endl;
    }

    queue4.enqueue("alpha");
    queue4.enqueue("beta");
    queue4.enqueue("gamma");

    string first = queue4.dequeue();
    if (first != "alpha") {
        cerr << "First dequeue: expected \"alpha\", got \"" << first << "\"" << endl;
        passed = false;
    } else {
        cout << "  First dequeue = \"alpha\"" << endl;
    }

    string second = queue4.dequeue();
    if (second != "beta") {
        cerr << "Second dequeue: expected \"beta\", got \"" << second << "\"" << endl;
        passed = false;
    } else {
        cout << "  Second dequeue = \"beta\"" << endl;
    }

    string third = queue4.dequeue();
    if (third != "gamma") {
        cerr << "Third dequeue: expected \"gamma\", got \"" << third << "\"" << endl;
        passed = false;
    } else {
        cout << "  Third dequeue = \"gamma\"" << endl;
    }

    string empty = queue4.dequeue();
    if (empty != "") {
        cerr << "Dequeue from empty: expected empty string, got \"" << empty << "\"" << endl;
        passed = false;
    } else {
        cout << "  Dequeue from empty queue = \"\" (empty string)" << endl;
    }

    // Test processBatch
    cout << "\nTesting processBatch..." << endl;
    MessageQueue queue5;
    queue5.enqueue("1");
    queue5.enqueue("2");
    queue5.enqueue("3");
    queue5.enqueue("4");
    queue5.enqueue("5");

    vector<string> batch1 = processBatch(queue5, 2);
    if (batch1.size() != 2) {
        cerr << "processBatch(queue, 2): expected 2 items, got " << batch1.size() << endl;
        passed = false;
    } else if (batch1[0] != "1" || batch1[1] != "2") {
        cerr << "processBatch should return [\"1\", \"2\"], got [\"" << batch1[0] << "\", \"" << batch1[1] << "\"]" << endl;
        passed = false;
    } else {
        cout << "  processBatch(queue, 2) = [\"1\", \"2\"]" << endl;
    }

    if (queue5.size() != 3) {
        cerr << "Queue should have 3 items left, got " << queue5.size() << endl;
        passed = false;
    } else {
        cout << "  Queue has 3 items remaining" << endl;
    }

    vector<string> batch2 = processBatch(queue5, 5);
    if (batch2.size() != 3) {
        cerr << "processBatch(queue, 5): expected 3 items, got " << batch2.size() << endl;
        passed = false;
    } else if (batch2[0] != "3" || batch2[1] != "4" || batch2[2] != "5") {
        cerr << "processBatch should return [\"3\", \"4\", \"5\"]" << endl;
        passed = false;
    } else {
        cout << "  processBatch(queue, 5) = [\"3\", \"4\", \"5\"] (only 3 available)" << endl;
    }

    // Test edge cases
    cout << "\nTesting edge cases..." << endl;
    MessageQueue queue6;
    vector<string> emptyBatch = processBatch(queue6, 10);
    if (emptyBatch.size() != 0) {
        cerr << "processBatch on empty queue: expected 0 items, got " << emptyBatch.size() << endl;
        passed = false;
    } else {
        cout << "  processBatch on empty queue = [] (empty vector)" << endl;
    }

    // Test size tracking after mixed operations
    cout << "\nTesting size tracking after mixed operations..." << endl;
    MessageQueue queue7;
    queue7.enqueue("a");
    queue7.enqueue("b");
    queue7.dequeue();
    queue7.enqueue("c");
    queue7.dequeue();

    if (queue7.size() != 1) {
        cerr << "Size after mixed operations: expected 1, got " << queue7.size() << endl;
        passed = false;
    } else {
        cout << "  Size after enqueue(a), enqueue(b), dequeue, enqueue(c), dequeue = 1" << endl;
    }

    string remaining = queue7.peek();
    if (remaining != "c") {
        cerr << "Remaining element should be \"c\", got \"" << remaining << "\"" << endl;
        passed = false;
    } else {
        cout << "  Remaining element is \"c\"" << endl;
    }

    if (passed) {
        cout << "\nAll tests passed!" << endl;
        return 0;
    } else {
        cout << "\nSome tests failed." << endl;
        return 1;
    }
}
