# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from pytopi import PyTopiNetwork, zeros
from time import sleep

def test_basic():
    network = basic_network()
    arr = zeros((3,4))
    network.send('Eagle', arr)
    (arr, _) = network.recv()
    assert(arr[0,0] == 1.0)
    arr[0,0] = 10.
    network.send('Eagle', arr)

def test_try_recv():
    network = basic_network()
    assert(network.try_recv()[0] is None)
    arr = zeros((3,4))
    network.send('Eagle', arr)
    sleep(1)
    (arr, _) = network.try_recv()
    assert(arr is not None)
    assert(arr[0,0] == 1.0)
    arr[0,0] = 10.
    network.send('Eagle', arr)

def basic_network():
    node = """
import numpy as np
import topi

def topi_entrypoint(messenger):
    while True:
        arr, _ = messenger.recv()
        if (arr[0,0] == 10.):
            break
        arr[0,0] += 1.
        messenger.send('Base', arr)
    """
    network = PyTopiNetwork('Base', [(-1, 'Eagle', node)], None)
    return network
    