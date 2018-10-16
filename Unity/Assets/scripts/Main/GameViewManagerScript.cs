﻿//Esteban

using System;
using System.Collections;
using System.Runtime.InteropServices;
using UnityEngine;
using UnityEngine.UI;


public class GameViewManagerScript : MonoBehaviour
{
	public Transform P1Transform;
	public Transform P1Hands;

	public GameObject UI;
	public Text final_score_P1;
	public Text final_score_P2;
	public Text wl_p1;
	public Text wl_p2;

	
	public Transform P2Transform;
	public Transform P2Hands;

	public Transform FrisbeeTransform;
	private bool frisbeeHeld = false;
	private int timetoend;
	public TextMesh Timer;
	public TextMesh P1Score;
	
	public TextMesh P2Score;
	
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
		public sbyte zbee_held;

		public double time;

		public sbyte State;
	}

	private ManagedState MState;
	
	[DllImport("rustjammers_engine")]
	static extern void send_type_p1(IntPtr gameEngine,sbyte type1);
	
	[DllImport("rustjammers_engine")]
	static extern void send_type_p2(IntPtr gameEngine,sbyte type2);
	
	[DllImport("rustjammers_engine")]
	static extern IntPtr initialize();
	
	[DllImport("rustjammers_engine")]
	static extern void epoch(IntPtr gameEngine, sbyte p1_h_action, sbyte p2_h_action);
	
	[DllImport("rustjammers_engine")]
	static extern ManagedState get_state(IntPtr gameEngine);
	
	[DllImport("rustjammers_engine")]
	static extern void reset(IntPtr gameEngine);
	
	[DllImport("rustjammers_engine")]
	static extern void dispose(IntPtr gameEngine);

	private IntPtr currentGameEngine;

	private HumanInput[] inputs = new HumanInput[2];
	
	[Flags]
	public enum HumanInput
	{
		Idle = 0,
		Up = 1,
		Down = 2,
		Left = 4,
		Right = 8,
		Throw = 16,
	}

	void Start()
	{
		UI.SetActive(false);
		Debug.Log(DateTime.Now.ToString("yyyy/MM/dd HH:mm:ss.ff") + " - Initializing engine...");
		currentGameEngine = initialize();
		reset(currentGameEngine);
		MState =new ManagedState();
		send_type_p1(currentGameEngine, (sbyte)PlayerType.MyPlayersType.typeP1);
		send_type_p2(currentGameEngine, (sbyte)PlayerType.MyPlayersType.typeP2);
		Debug.Log(DateTime.Now.ToString("yyyy/MM/dd HH:mm:ss.ff") + " - Engine ready [" + currentGameEngine + "].");
		Debug.Log(MState.State);
	}

	private void CollectInput(int index) {
		var inputManagerIndex = (index + 1).ToString();

		var horizontal = Input.GetAxisRaw("P" + inputManagerIndex + "Horizontal");
		if (horizontal < 0.0) {
			inputs[index] |= HumanInput.Left;
		} else if (horizontal > 0.0) {
			inputs[index] |= HumanInput.Right;
		}

		var vertical = Input.GetAxisRaw("P" + inputManagerIndex + "Vertical");
		if (vertical < 0.0) {
			inputs[index] |= HumanInput.Down;
		} else if (vertical > 0.0) {
			inputs[index] |= HumanInput.Up;
		}

		if (Input.GetButtonDown("P" + inputManagerIndex + "Throw")) {
			inputs[index] |= HumanInput.Throw;
		}
	}
	
	void Update () {
		inputs[0] = HumanInput.Idle;
		inputs[1] = HumanInput.Idle;

		if (PlayerType.MyPlayersType.typeP1 == PlayerType.AgentType.Human)
		{
			CollectInput(0);
		}
		if (PlayerType.MyPlayersType.typeP2 == PlayerType.AgentType.Human)
		{
			CollectInput(1);
		}
		
		epoch(currentGameEngine, (sbyte)inputs[0], (sbyte)inputs[1]);
		MState = get_state(currentGameEngine);
		if (MState.p1_score < 10)
		{
			P1Score.text = "0"+MState.p1_score;
		}
		else
		{
			P1Score.text = ""+MState.p1_score;
		}
		if (MState.p2_score < 10)
		{
			P2Score.text = "0"+MState.p2_score;
		}
		else
		{
			P2Score.text = ""+MState.p2_score;
		}
		P1Transform.position = new Vector3((float)MState.p1_x, P1Transform.position.y,(float)MState.p1_y);
		P2Transform.position = new Vector3((float)MState.p2_x, P2Transform.position.y,(float)MState.p2_y);
		if (!frisbeeHeld) {
			if (MState.zbee_held == 0) {
				FrisbeeTransform.parent = P1Hands;
				FrisbeeTransform.localPosition = Vector3.zero;
				frisbeeHeld = true;
			} else if (this.MState.zbee_held == 1) {
				FrisbeeTransform.parent = P2Hands;
				FrisbeeTransform.localPosition = Vector3.zero;
				frisbeeHeld = true;
			}
		} else {
			if (MState.zbee_held == -1) {
				FrisbeeTransform.parent = null;
				frisbeeHeld = false;
			}
		}
		if (!frisbeeHeld) {
			FrisbeeTransform.position = new Vector3((float)MState.zbee_x, P2Transform.position.y,(float)MState.zbee_y);
		}
		if (MState.State != 2)
		{
			var roundedTime = Mathf.RoundToInt((float) MState.time);
			if (roundedTime < 10)
			{
				Timer.text = "0" + roundedTime;
			}
			else
			{
				Timer.text = roundedTime.ToString();
			}
		}
		else
		{
			UI.SetActive(true);
			if (MState.p1_score < 10)
			{
				final_score_P1.text = "0" + MState.p1_score;
			}
			else
			{
				final_score_P1.text = "" + MState.p1_score;
			}
			if (MState.p2_score < 10)
			{
				final_score_P2.text = "0" + MState.p2_score;
			}
			else
			{
				final_score_P2.text = "" + MState.p2_score;
			}
			if (MState.p2_score > MState.p1_score)
			{
				wl_p1.text = "Looser";
				wl_p2.text = "Winner";
			}
			if (MState.p2_score < MState.p1_score)
			{
				wl_p1.text = "Winner";
				wl_p2.text = "Looser";
			}
			if (MState.p2_score == MState.p1_score)
			{
				wl_p1.text = "Draw";
				wl_p2.text = "Draw";
			}
		}
	}


	private void OnDestroy()
	{
		Debug.Log(DateTime.Now.ToString("yyyy/MM/dd HH:mm:ss.ff") + " - Destroying engine...");
		dispose(currentGameEngine);
	}
}

