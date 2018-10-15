﻿//Esteban

using System;
using System.Runtime.InteropServices;
using UnityEngine;
using UnityEngine.UI;


public class GameViewManagerScript : MonoBehaviour
{
	public Transform P1Transform;
	
	public Transform P2Transform;
	
	public Transform ZbeeTransform;

	public Text P1Score;
	
	public Text P2Score;
	
	[StructLayout(LayoutKind.Sequential)]
	public struct ManagedState
	{
		public double p1_x;
		public double p1_y;
		public sbyte p1_score;

		public double p2_x;
		public double p2_y;
		public sbyte p2_score;

		public double zbee_x;
		public double zbee_y;
	}

	private ManagedState MState;
	
	[DllImport("rustjammers_engine")]
	static extern void sendtypeP1(int type1);
	
	[DllImport("rustjammers_engine")]
	static extern void sendtypeP2(int type2);
	
	[DllImport("rustjammers_engine")]
	static extern IntPtr initialize();
	
	[DllImport("rustjammers_engine")]
	static extern void epoch(IntPtr gameEngine);
	
	[DllImport("rustjammers_engine")]
	static extern ManagedState get_state(IntPtr gameEngine);
	
	[DllImport("rustjammers_engine")]
	static extern void reset(IntPtr gameEngine);
	
	[DllImport("rustjammers_engine")]
	static extern void dispose(IntPtr gameEngine);

	private IntPtr currentGameEngine;
	
	
	void Start()
	{
		currentGameEngine = initialize();
		reset(currentGameEngine);
		MState =new ManagedState();
		sendtypeP1(PlayerType.MyPlayersType.typeP1);
		sendtypeP2(PlayerType.MyPlayersType.typeP2);
	}
	
	void Update () {
		
		epoch(currentGameEngine);
		MState = get_state(currentGameEngine);
		P1Score.text = ""+MState.p1_score;
		P2Score.text = ""+MState.p2_score;
		P1Transform.position = new Vector3((float)MState.p1_x, P1Transform.position.y,(float)MState.p1_y);
		P2Transform.position = new Vector3((float)MState.p2_x, P2Transform.position.y,(float)MState.p2_y);
		ZbeeTransform.position = new Vector3((float)MState.zbee_x, P2Transform.position.y,(float)MState.zbee_y);
		
	}


	private void OnDestroy()
	{
		dispose(currentGameEngine);
	}
}

